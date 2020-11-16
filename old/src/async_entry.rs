use crate::error::*;
use crate::event::*;
use crate::gstr;
use crate::gstr::*;
use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::executor::LocalPool;
use futures::lock::{Mutex, MutexGuard};
use futures::prelude::*;
use std::ptr;
use std::string::String;
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

/// dllmain entry.
/// Save hinst only.
#[allow(dead_code)]
pub fn dllmain(hinst: usize, ul_reason_for_call: DWORD, _lp_reserved: LPVOID) -> bool {
    match ul_reason_for_call {
        DLL_PROCESS_ATTACH => {
            set_hinst(hinst);
            true
        }
        DLL_PROCESS_DETACH => unload_sync(),
        _ => true,
    }
}

/// set H_INST
pub fn set_hinst(hinst: usize) {
    unsafe {
        H_INST = hinst;
    }
}

/// load entry.
/// 1. Create shiori3 event stream.
/// 2. Send Load Event.
#[allow(dead_code)]
pub fn load(hdir: HGLOBAL, len: usize) -> ApiResult<EventReceiver> {
    LocalPool::new().run_until(async { load_impl(hdir, len).await })
}

/// unload entry.
/// 1. Send Unload Event.
/// 2. Wait event.result.
/// 3. Drop shiori3 event stream.
#[allow(dead_code)]
pub fn unload() -> bool {
    LocalPool::new().run_until(async {
        match unload_impl().await {
            Err(e) => {
                error!("{:?}", e);
                false
            }
            _ => true,
        }
    })
}

/// request entry.
/// 1. Send Request Event.
/// 2. Wait event.result.
#[allow(dead_code)]
pub fn request(hreq: HGLOBAL, len: &mut usize) -> HGLOBAL {
    LocalPool::new().run_until(async {
        match request_impl(hreq, len).await {
            Err(e) => {
                error!("{:?}", e);
                *len = 0;
                ptr::null_mut()
            }
            Ok(res) => {
                let res = gstr::clone_from_str_nofree(&*res);
                *len = res.len();
                res.handle()
            }
        }
    })
}

static mut H_INST: usize = 0;

async fn lock_sender<'a>() -> ApiResult<MutexGuard<'a, Option<EventSender>>> {
    lazy_static! {
        static ref SENDER: Mutex<Option<EventSender>> = Mutex::new(None);
    }
    Ok(SENDER.lock().await)
}

#[allow(dead_code)]
const DLL_PROCESS_DETACH: DWORD = 0;
#[allow(dead_code)]
const DLL_PROCESS_ATTACH: DWORD = 1;
#[allow(dead_code)]
const DLL_THREAD_ATTACH: DWORD = 2;
#[allow(dead_code)]
const DLL_THREAD_DETACH: DWORD = 3;

/// DLL_PROCESS_DETACH 処理：何もしない。
/// unloadは非同期実装なのでこのタイミングでは呼び出せない。
pub fn unload_sync() -> bool {
    true
}

async fn load_impl(hdir: HGLOBAL, len: usize) -> ApiResult<EventReceiver> {
    // load済みなら解放
    let _ = unload_impl().await;
    // create api
    let (tx, rx) = mpsc::channel::<Event>(16);
    let mut lock_sender = lock_sender().await?;
    *lock_sender = Some(tx);

    // send event
    let hinst = unsafe { H_INST };
    let load_dir = gstr::capture_path(hdir, len);
    let sender = lock_sender.as_mut().ok_or(ApiError::NotLoad)?;
    sender.send(Event::Load(Load { hinst, load_dir })).await?;

    Ok(rx)
}

async fn unload_impl() -> ApiResult<()> {
    let rc = unload_send().await;
    let _ = unload_drop().await?;
    rc
}
async fn unload_send() -> ApiResult<()> {
    // send event
    let rx = {
        let mut lock_sender = lock_sender().await?;
        let sender = lock_sender.as_mut().ok_or(ApiError::NotLoad)?;
        let (tx, rx) = oneshot::channel::<ApiResult<()>>();
        sender
            .send(Event::Unload(Unload {
                res: EventResponse(tx),
            }))
            .await?;
        rx
    };

    // wait result
    rx.await?
}
async fn unload_drop() -> ApiResult<()> {
    let mut lock_sender = lock_sender().await?;
    *lock_sender = None;
    Ok(())
}

async fn request_impl(hreq: HGLOBAL, len: &mut usize) -> ApiResult<String> {
    // send event
    let rx = {
        let mut lock_sender = lock_sender().await?;
        let sender = lock_sender.as_mut().ok_or(ApiError::NotLoad)?;
        let req = gstr::capture_str(hreq, *len);
        let (tx, rx) = oneshot::channel::<ApiResult<String>>();
        sender
            .send(Event::Request(Request {
                req,
                res: EventResponse(tx),
            }))
            .await?;
        rx
    };

    // wait result
    rx.await?
}

impl From<mpsc::SendError> for ApiError {
    fn from(_: mpsc::SendError) -> ApiError {
        ApiError::EventSendError
    }
}
impl From<oneshot::Canceled> for ApiError {
    fn from(_: oneshot::Canceled) -> ApiError {
        ApiError::EventCanceled
    }
}
