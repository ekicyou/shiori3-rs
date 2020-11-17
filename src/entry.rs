use crate::gstr;
use crate::prelude::*;
use log::*;
use once_cell::sync::Lazy;
use std::ptr;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

static mut H_INST: usize = 0;

/// set H_INST
fn set_hinst(hinst: usize) {
    unsafe {
        H_INST = hinst;
    }
}

/// get H_INST
fn hinst() -> usize {
    H_INST
}

static mut API: Lazy<Mutex<Rc<Option<Sender<ShioriEvent>>>>> =
    Lazy::new(|| Mutex::new(Rc::new(None)));

/// register ch api.
pub fn register(request_sender: Sender<ShioriEvent>) -> ApiResult<()> {
    let mut lock = (*API).lock()?;
    **lock = Some(request_sender);
    Ok(())
}

/// unregister ch api.
fn unregister() -> ApiResult<()> {
    let mut lock = (*API).lock()?;
    **lock = None;
    Ok(())
}
/// send ch api.
fn send(ev: ShioriEvent) -> ApiResult<()> {
    let lock = (*API).lock()?;
    match **lock {
        Some(a) => a.send(ev)?,
        _ => Err(ApiError::NotLoad)?,
    }
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

/// dllmain entry.
/// Save hinst only.
#[allow(dead_code)]
pub fn dllmain(hinst: usize, ul_reason_for_call: DWORD, _lp_reserved: LPVOID) -> bool {
    match ul_reason_for_call {
        DLL_PROCESS_ATTACH => {
            set_hinst(hinst);
            true
        }
        DLL_PROCESS_DETACH => unload(),
        _ => true,
    }
}

/// load entry.
/// 1. Create shiori3 event stream.
/// 2. Send Load Event.
#[allow(dead_code)]
pub fn load(hdir: HGLOBAL, len: usize) -> bool {
    match load_impl(hdir, len) {
        Err(e) => {
            error!("{:?}", e);
            false
        }
        _ => true,
    }
}
fn load_impl(hdir: HGLOBAL, len: usize) -> ApiResult<()> {
    let load_dir = gstr::capture_path(hdir, len);
    let ev = ShioriEvent::Load(hinst(), load_dir);
    send(ev)?;
    Ok(())
}

/// unload entry.
/// 1. Send Unload Event.
/// 2. Drop shiori3 event stream.
#[allow(dead_code)]
pub fn unload() -> bool {
    match unload_impl() {
        Err(e) => {
            error!("{:?}", e);
            false
        }
        _ => true,
    }
}
fn unload_impl() -> ApiResult<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel::<()>(0);
    let ev = ShioriEvent::Unload(tx);
    send(ev)?;
    let _ = rx.recv()?;
    unregister();
    Ok(())
}

/// request entry.
/// 1. Send Request Event.
/// 2. Wait event.result.
#[allow(dead_code)]
pub fn request(hreq: HGLOBAL, len: &mut usize) -> HGLOBAL {
    let req = gstr::capture_str(hreq, *len);
    match request_impl(req) {
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
}
pub fn request_impl(req: GCowStr) -> ApiResult<String> {
    let args = RequestArgs::new(req)?;
    unimplemented!();
}
