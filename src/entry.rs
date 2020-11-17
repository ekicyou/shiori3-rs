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

struct ShioriEntryStore<TAPI: ShioriAPI> {
    api: TAPI,
}

pub struct ShioriEntry<TAPI: ShioriAPI>(Mutex<ShioriEntryStore<TAPI>>);

#[allow(dead_code)]
#[allow(non_snake_case)]
const DLL_PROCESS_DETACH: DWORD = 0;
#[allow(dead_code)]
#[allow(non_snake_case)]
const DLL_PROCESS_ATTACH: DWORD = 1;
#[allow(dead_code)]
#[allow(non_snake_case)]
const DLL_THREAD_ATTACH: DWORD = 2;
#[allow(dead_code)]
#[allow(non_snake_case)]
const DLL_THREAD_DETACH: DWORD = 3;

impl<TAPI: ShioriAPI> ShioriEntry<TAPI> {
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
    pub fn load(&mut self, hdir: HGLOBAL, len: usize) -> bool {
        match self.load_impl(hdir, len) {
            Err(e) => {
                error!("{:?}", e);
                false
            }
            _ => true,
        }
    }
    fn load_impl(&mut self, hdir: HGLOBAL, len: usize) -> ApiResult<()> {
        let mut lock = self.0.lock()?;
        let load_dir = gstr::capture_path(hdir, len);
        (*lock).api.load(hinst(), load_dir)
    }

    /// unload entry.
    /// 1. Send Unload Event.
    /// 2. Drop shiori3 event stream.
    #[allow(dead_code)]
    pub fn unload(&mut self) -> bool {
        match self.unload_impl() {
            Err(e) => {
                error!("{:?}", e);
                false
            }
            _ => true,
        }
    }
    fn unload_impl(&mut self) -> ApiResult<()> {
        let mut lock = self.0.lock()?;
        (*lock).api.unload()
    }

    /// request entry.
    /// 1. Send Request Event.
    /// 2. Wait event.result.
    #[allow(dead_code)]
    pub fn request(&mut self, hreq: HGLOBAL, len: &mut usize) -> HGLOBAL {
        let req = gstr::capture_str(hreq, *len);
        match self.request_impl(req) {
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
    fn request_impl(&mut self, req: GCowStr) -> ApiResult<String> {
        let mut lock = self.0.lock()?;
        (*lock).api.request(req)
    }
}
