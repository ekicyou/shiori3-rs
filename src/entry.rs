use crate::prelude::*;
use log::*;
use once_cell::sync::Lazy;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

#[allow(dead_code)]
const DLL_PROCESS_DETACH: DWORD = 0;
#[allow(dead_code)]
const DLL_PROCESS_ATTACH: DWORD = 1;
#[allow(dead_code)]
const DLL_THREAD_ATTACH: DWORD = 2;
#[allow(dead_code)]
const DLL_THREAD_DETACH: DWORD = 3;

static mut H_INST: usize = 0;
static mut API: Lazy<Mutex<Rc<Option<Sender<ShioriEvent>>>>> =
    Lazy::new(|| Mutex::new(Rc::new(None)));

/// DI api.
pub fn register(request_sender: Sender<ShioriEvent>) -> ApiResult<()> {
    let mut lock = (*API).lock()?;
    **lock = Some(request_sender);
    Ok(())
}

fn send_api(ev: ShioriEvent) -> ApiResult<()> {
    let lock = (*API).lock()?;
    match *lock {
        Some(a) => a.send(ev)?,
        _ => Err(ApiError::NotLoad)?,
    }
    Ok(())
}

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

/// set H_INST
fn set_hinst(hinst: usize) {
    unsafe {
        H_INST = hinst;
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
fn load_impl(hdir: HGLOBAL, len: usize) -> ApiResult<()> {}

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
