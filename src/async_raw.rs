use crate::hglobal::GStr;
use futures::channel::oneshot::{Receiver, Sender};
use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::sync::{Mutex, MutexGuard, PoisonError};
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

#[derive(Debug, PartialEq)]
pub enum Error {
    Unimplemented,
    PoisonError,
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Error {
        Error::PoisonError
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Load {
    pub h_inst: usize,
    pub load_dir: GStr,
    pub res: Sender<Result<()>>,
}

pub struct Unload {
    pub res: Sender<Result<()>>,
}

pub struct Request {
    pub req: GStr,
    pub res: Sender<Result<GStr>>,
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

pub struct RawEntry {}

#[allow(dead_code)]
const DLL_PROCESS_DETACH: DWORD = 0;
#[allow(dead_code)]
const DLL_PROCESS_ATTACH: DWORD = 1;
#[allow(dead_code)]
const DLL_THREAD_ATTACH: DWORD = 2;
#[allow(dead_code)]
const DLL_THREAD_DETACH: DWORD = 3;

static mut H_INST: usize = 0;
lazy_static! {
    static ref WORK: Mutex<RawEntry> = Mutex::new(init_work());
}
fn init_work() -> RawEntry {
    RawEntry {}
}
fn work<'a>() -> std::result::Result<
    std::sync::MutexGuard<'a, RawEntry>,
    std::sync::PoisonError<std::sync::MutexGuard<'a, RawEntry>>,
> {
    WORK.lock()
}

impl RawEntry {
    #[allow(dead_code)]
    pub fn dllmain(hinst: usize, ul_reason_for_call: DWORD, _lp_reserved: LPVOID) -> bool {
        match ul_reason_for_call {
            DLL_PROCESS_ATTACH => {
                unsafe {
                    H_INST = hinst;
                }
                true
            }
            DLL_PROCESS_DETACH => Self::unload(),
            _ => true,
        }
    }

    #[allow(dead_code)]
    pub fn load(hdir: HGLOBAL, len: usize) -> Result<RawAPI> {
        Self::unload();
        let mut work = work()?;
        unimplemented!();
    }

    #[allow(dead_code)]
    pub fn unload() -> bool {
        match RawEntry::unload_impl() {
            Err(e) => {
                error!("{:?}", e);
                false
            }
            _ => true,
        }
    }
    fn unload_impl() -> Result<()> {
        let mut work = work()?;
        unimplemented!();
    }

    #[allow(dead_code)]
    pub fn request(hreq: HGLOBAL, len: &mut usize) -> HGLOBAL {
        match RawEntry::request_impl(hreq, len) {
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
    fn request_impl(hreq: HGLOBAL, len: &mut usize) -> Result<GStr> {
        let mut work = work()?;
        unimplemented!();
    }
}

pub struct RawAPI {}

impl RawAPI {
    fn new(hinst: usize) -> Result<RawAPI> {
        unimplemented!();
    }

    pub async fn run<F>(self: Pin<&mut Self>, feature: F)
    where
        F: Fn(Pin<&mut Self>) -> Future<Output = Result<()>>,
    {
        unimplemented!();
    }

    pub async fn event(self: Pin<&mut Self>) -> Result<Event> {
        unimplemented!();
    }
}
