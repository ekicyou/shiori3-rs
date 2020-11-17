use crate::gstr;
use crate::prelude::*;
use log::*;
use std::ptr;
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
    unsafe { H_INST }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
pub const DLL_PROCESS_DETACH: DWORD = 0;
#[allow(dead_code)]
#[allow(non_snake_case)]
pub const DLL_PROCESS_ATTACH: DWORD = 1;
#[allow(dead_code)]
#[allow(non_snake_case)]
pub const DLL_THREAD_ATTACH: DWORD = 2;
#[allow(dead_code)]
#[allow(non_snake_case)]
pub const DLL_THREAD_DETACH: DWORD = 3;

/// dllmain entry.
/// Save hinst only.
#[allow(dead_code)]
pub fn dllmain(hinst: usize, ul_reason_for_call: DWORD, _lp_reserved: LPVOID) -> bool {
    match ul_reason_for_call {
        DLL_PROCESS_ATTACH => {
            set_hinst(hinst);
            true
        }
        //DLL_PROCESS_DETACH => unload(),
        _ => true,
    }
}

/// load entry.
/// 1. Create shiori3 event stream.
/// 2. Send Load Event.
#[allow(dead_code)]
pub fn load<T: ShioriAPI>(api: &Mutex<T>, hdir: HGLOBAL, len: usize) -> bool {
    match impl_load(api, hdir, len) {
        Err(e) => {
            error!("{:?}", e);
            false
        }
        _ => true,
    }
}
fn impl_load<T: ShioriAPI>(api: &Mutex<T>, hdir: HGLOBAL, len: usize) -> ApiResult<()> {
    let mut lock = api.lock()?;
    let load_dir = gstr::capture_path(hdir, len);
    (*lock).load(hinst(), load_dir)
}

/// unload entry.
/// 1. Send Unload Event.
/// 2. Drop shiori3 event stream.
#[allow(dead_code)]
pub fn unload<T: ShioriAPI>(api: &Mutex<T>) -> bool {
    match unload_impl(api) {
        Err(e) => {
            error!("{:?}", e);
            false
        }
        _ => true,
    }
}
fn unload_impl<T: ShioriAPI>(api: &Mutex<T>) -> ApiResult<()> {
    let mut lock = api.lock()?;
    (*lock).unload()
}

/// request entry.
/// 1. Send Request Event.
/// 2. Wait event.result.
#[allow(dead_code)]
pub fn request<T: ShioriAPI>(api: &Mutex<T>, hreq: HGLOBAL, len: &mut usize) -> HGLOBAL {
    let req = gstr::capture_str(hreq, *len);
    match request_impl(api, req) {
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
fn request_impl<T: ShioriAPI>(api: &Mutex<T>, req: GCowStr) -> ApiResult<String> {
    let mut lock = api.lock()?;
    (*lock).request(req)
}
