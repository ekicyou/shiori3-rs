use crate::gstr::GStr;
use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::executor::LocalPool;
use futures::lock::{Mutex, MutexGuard};
use std::ptr;
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotLoad,
    PoisonError,
    EventTrySendError,
    EventNotInitialized,
    EventCanceled,
    Shutdowned,
}

impl From<mpsc::TrySendError<Event>> for Error {
    fn from(_: mpsc::TrySendError<Event>) -> Error {
        Error::EventTrySendError
    }
}
impl From<oneshot::Canceled> for Error {
    fn from(_: oneshot::Canceled) -> Error {
        Error::EventCanceled
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Load {
    pub hinst: usize,
    pub load_dir: GStr,
}

pub struct Unload {
    pub res: oneshot::Sender<Result<()>>,
}

pub struct Request {
    pub req: GStr,
    pub res: oneshot::Sender<Result<GStr>>,
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

static mut H_INST: usize = 0;
lazy_static! {
    static ref EVENT_SENDER: Mutex<Option<EventSender>> = Mutex::new(None);
}

async fn lock_sender<'a>() -> Result<MutexGuard<'a, Option<EventSender>>> {
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
pub fn load(hdir: HGLOBAL, len: usize) -> Result<EventReceiver> {
    let mut pool = LocalPool::new();
    pool.run_until(async { load_impl(hdir, len).await })
}

/// unload処理。終了を待機して結果を返す。
#[allow(dead_code)]
pub fn unload() -> bool {
    let mut pool = LocalPool::new();
    pool.run_until(async {
        match unload_impl().await {
            Err(e) => {
                error!("{:?}", e);
                false
            }
            _ => true,
        }
    })
}

/// DLL_PROCESS_DETACH 処理：何もしない。               
/// unloadは非同期実装なのでこのタイミングでは呼び出せない。
pub fn unload_sync() -> bool {
    true
}

/// request処理。apiにRequestイベントを発行し、結果を待機して返す。
#[allow(dead_code)]
pub fn request(hreq: HGLOBAL, len: &mut usize) -> HGLOBAL {
    let mut pool = LocalPool::new();
    pool.run_until(async {
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

async fn load_impl(hdir: HGLOBAL, len: usize) -> Result<EventReceiver> {
    unload_impl().await?;
    // create api
    let (tx, rx) = mpsc::channel::<Event>(16);
    let mut lock_sender = lock_sender().await?;
    *lock_sender = Some(tx);

    // send event
    let hinst = unsafe { H_INST };
    let gdir = GStr::capture(hdir, len);
    let sender = lock_sender.as_mut().ok_or(Error::NotLoad)?;
    sender.try_send(Event::Load(Load {
        hinst: hinst,
        load_dir: gdir,
    }))?;

    Ok(rx)
}

async fn unload_impl() -> Result<()> {
    // send event
    let rx = {
        let mut lock_sender = lock_sender().await?;
        let sender = lock_sender.as_mut().ok_or(Error::NotLoad)?;
        let (tx, rx) = oneshot::channel::<Result<()>>();
        let send_rc = sender.try_send(Event::Unload(Unload { res: tx }));
        *lock_sender = None;
        send_rc?;
        rx
    };

    // wait result
    rx.await?
}

async fn request_impl(hreq: HGLOBAL, len: &mut usize) -> Result<GStr> {
    // send event
    let rx = {
        let mut lock_sender = lock_sender().await?;
        let sender = lock_sender.as_mut().ok_or(Error::NotLoad)?;
        let greq = GStr::capture(hreq, *len);
        let (tx, rx) = oneshot::channel::<Result<GStr>>();
        sender.try_send(Event::Request(Request { req: greq, res: tx }))?;
        rx
    };

    // wait result
    rx.await?
}
