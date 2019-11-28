use crate::hglobal::GStr;
use futures::channel::oneshot::{Receiver, Sender};
use std::future::Future;
use std::pin::Pin;
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

pub enum Error {
    NotImp,
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

pub struct RawAPI {}

impl RawAPI {
    #[allow(dead_code)]
    pub unsafe fn entry_dllmain(
        hinst: usize,
        ul_reason_for_call: DWORD,
        _lp_reserved: LPVOID,
    ) -> bool {
        unimplemented!();
    }

    #[allow(dead_code)]
    pub unsafe fn entry_load<F>(hdir: HGLOBAL, len: usize, feature: F) -> bool
    where
        F: Fn(Pin<&mut Self>) -> Future<Output = Result<()>>,
    {
        unimplemented!();
    }

    #[allow(dead_code)]
    pub unsafe fn entry_unload() -> bool {
        unimplemented!();
    }

    #[allow(dead_code)]
    pub unsafe fn entry_request(hreq: HGLOBAL, len: &mut usize) -> HGLOBAL {
        unimplemented!();
    }

    pub async fn event(self: Pin<&mut Self>) -> Result<Event> {
        unimplemented!();
    }
}
