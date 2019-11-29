use crate::gstr::GStr;
use finally_block::finally;
use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::stream::StreamExt;
use std::pin::Pin;
use std::ptr;
use std::sync::{Mutex, MutexGuard, PoisonError};
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

#[derive(Debug, PartialEq)]
pub enum Error {
    Unimplemented,
    PoisonError,
    EventTrySendError,
    EventNotInitialized,
    EventCanceled,
    Shutdowned,
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Error {
        Error::PoisonError
    }
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

pub struct RawEntry {
    event_sender: mpsc::Sender<Event>,
}

fn init_work(event_sender: mpsc::Sender<Event>) -> RawEntry {
    RawEntry {
        event_sender: event_sender,
    }
}

static mut H_INST: usize = 0;
lazy_static! {
    static ref WORK: Mutex<Option<RawEntry>> = Mutex::new(None);
}
/// ワークへのロックを取得します。
fn work_mutex<'a>() -> Result<MutexGuard<'a, Option<RawEntry>>> {
    Ok(WORK.lock()?)
}

/// ワークを破棄します。
fn drop_work() -> Result<()> {
    let mut lock_work = work_mutex()?;
    *lock_work = None;
    Ok(())
}

#[allow(dead_code)]
const DLL_PROCESS_DETACH: DWORD = 0;
#[allow(dead_code)]
const DLL_PROCESS_ATTACH: DWORD = 1;
#[allow(dead_code)]
const DLL_THREAD_ATTACH: DWORD = 2;
#[allow(dead_code)]
const DLL_THREAD_DETACH: DWORD = 3;

impl RawEntry {
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
            DLL_PROCESS_DETACH => Self::unload_sync(),
            _ => true,
        }
    }

    /// load処理。新たにapiを返す。
    #[allow(dead_code)]
    pub async fn load(hdir: HGLOBAL, len: usize) -> Result<RawAPI> {
        Self::load_impl(hdir, len).await
    }

    /// unload処理。終了を待機して結果を返す。
    #[allow(dead_code)]
    pub async fn unload() -> bool {
        match RawEntry::unload_impl().await {
            Err(e) => {
                error!("{:?}", e);
                false
            }
            _ => true,
        }
    }

    /// DLL_PROCESS_DETACH 処理：何もしない。
    /// unloadは非同期実装なのでこのタイミングでは呼び出せない。
    pub fn unload_sync() -> bool {
        true
    }

    /// request処理。apiにRequestイベントを発行し、結果を待機して返す。
    #[allow(dead_code)]
    pub async fn request(hreq: HGLOBAL, len: &mut usize) -> HGLOBAL {
        match RawEntry::request_impl(hreq, len).await {
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
    }

    async fn load_impl(hdir: HGLOBAL, len: usize) -> Result<RawAPI> {
        Self::unload().await;
        // create api
        let (tx, rx) = mpsc::channel::<Event>(16);
        let mut lock_work = work_mutex()?;
        *lock_work = Some(init_work(tx));
        let api = RawAPI::new(rx)?;

        // send event
        let hinst = unsafe { H_INST };
        let gdir = GStr::capture(hdir, len);
        let work = lock_work.as_mut().ok_or(Error::Unimplemented)?;
        work.send_event(Event::Load(Load {
            hinst: hinst,
            load_dir: gdir,
        }))?;

        Ok(api)
    }

    async fn unload_impl() -> Result<()> {
        let _f = finally(|| {
            let _ = drop_work();
        });

        // send event
        let rx = {
            let mut lock_work = work_mutex()?;
            let work = lock_work.as_mut().ok_or(Error::Unimplemented)?;
            let (tx, rx) = oneshot::channel::<Result<()>>();
            work.send_event(Event::Unload(Unload { res: tx }))?;
            rx
        };

        // wait result
        rx.await?
    }

    async fn request_impl(hreq: HGLOBAL, len: &mut usize) -> Result<GStr> {
        // send event
        let rx = {
            let mut lock_work = work_mutex()?;
            let work = lock_work.as_mut().ok_or(Error::Unimplemented)?;
            let greq = GStr::capture(hreq, *len);
            let (tx, rx) = oneshot::channel::<Result<GStr>>();
            work.send_event(Event::Request(Request { req: greq, res: tx }))?;
            rx
        };

        // wait result
        rx.await?
    }

    fn send_event(&mut self, event: Event) -> Result<()> {
        self.event_sender.try_send(event)?;
        Ok(())
    }
}

pub struct RawAPI {
    event_receiver: mpsc::Receiver<Event>,
}

impl RawAPI {
    fn new(event_receiver: mpsc::Receiver<Event>) -> Result<RawAPI> {
        Ok(RawAPI {
            event_receiver: event_receiver,
        })
    }

    pub async fn event(mut self: Pin<&mut Self>) -> Result<Event> {
        self.event_receiver.next().await.ok_or(Error::Shutdowned)
    }
}
