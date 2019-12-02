use crate::error::*;
use crate::gstr::GStr;
use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::executor::LocalPool;
use futures::lock::{Mutex, MutexGuard};
use futures::prelude::*;
use std::ptr;
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

pub struct Load {
    pub hinst: usize,
    pub load_dir: GStr,
}

pub struct Unload {
    pub res: oneshot::Sender<ApiResult<()>>,
}

pub struct Request {
    pub req: GStr,
    pub res: oneshot::Sender<ApiResult<GStr>>,
}

/// raw SHIORI3 Event
pub enum Event {
    /// load(h_dir: HGLOBAL, len: usize) -> bool
    Load(Load),

    /// unload() -> bool
    Unload(Unload),

    /// request(h: HGLOBAL, len: &mut usize) -> HGLOBAL
    Request(Request),
}

type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;

/// dllmain処理。hinstを保存するのみ。
#[allow(dead_code)]
pub fn dllmain(hinst: usize, ul_reason_for_call: DWORD, _lp_reserved: LPVOID) -> bool {
    match ul_reason_for_call {
        DLL_PROCESS_ATTACH => {
            unsafe {
                H_INST = hinst;
            }
            true
        }
        DLL_PROCESS_DETACH => unload_sync(),
        _ => true,
    }
}

/// load処理。event receiver(stream)を返す。
#[allow(dead_code)]
pub fn load(hdir: HGLOBAL, len: usize) -> ApiResult<EventReceiver> {
    LocalPool::new().run_until(async { load_impl(hdir, len).await })
}

/// unload処理。終了を待機して結果を返す。
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

/// request処理。apiにRequestイベントを発行し、結果を待機して返す。
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
                *len = res.len();
                res.handle()
            }
        }
    })
}

static mut H_INST: usize = 0;
lazy_static! {
    static ref EVENT_SENDER: Mutex<Option<EventSender>> = Mutex::new(None);
}

async fn lock_sender<'a>() -> ApiResult<MutexGuard<'a, Option<EventSender>>> {
    Ok(EVENT_SENDER.lock().await)
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
    unload_impl().await?;
    // create api
    let (tx, rx) = mpsc::channel::<Event>(16);
    let mut lock_sender = lock_sender().await?;
    *lock_sender = Some(tx);

    // send event
    let hinst = unsafe { H_INST };
    let load_dir = GStr::capture(hdir, len);
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
        let (res, rx) = oneshot::channel::<ApiResult<()>>();
        sender.send(Event::Unload(Unload { res })).await?;
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

async fn request_impl(hreq: HGLOBAL, len: &mut usize) -> ApiResult<GStr> {
    // send event
    let rx = {
        let mut lock_sender = lock_sender().await?;
        let sender = lock_sender.as_mut().ok_or(ApiError::NotLoad)?;
        let req = GStr::capture(hreq, *len);
        let (res, rx) = oneshot::channel::<ApiResult<GStr>>();
        sender.send(Event::Request(Request { req, res })).await?;
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
