use super::api::Shiori3;
use super::error::*;
use hglobal::GStr;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
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

pub struct RawAPI<TS: Shiori3> {
    shiori: Mutex<Option<TS>>,
    h_inst: AtomicUsize,
}

impl<TS: Shiori3> Default for RawAPI<TS> {
    fn default() -> RawAPI<TS> {
        RawAPI::<TS> {
            shiori: Mutex::new(None),
            h_inst: AtomicUsize::default(),
        }
    }
}

impl<TS: Shiori3> RawAPI<TS> {
    fn get_h_inst(&self) -> usize {
        self.h_inst.load(Ordering::Relaxed)
    }
    fn set_h_inst(&self, value: usize) {
        self.h_inst.store(value, Ordering::Relaxed)
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_load(&self, hdir: HGLOBAL, len: usize) -> bool {
        match self.load(hdir, len) {
            Err(e) => {
                error!("{}", e);
                false
            }
            _ => true,
        }
    }
    fn load(&self, h_dir: HGLOBAL, l_dir: usize) -> ShioriResult<()> {
        let mut locked = self.shiori.lock()?;
        *locked = None;
        let g_dir = GStr::capture(h_dir, l_dir);
        let dir = g_dir.to_ansi_str()?;
        let shiori = TS::new(self.get_h_inst(), dir)?;
        *locked = Some(shiori);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_unload(&self) -> bool {
        match self.unload() {
            Err(e) => {
                error!("{}", e);
                false
            }
            _ => true,
        }
    }
    fn unload(&self) -> ShioriResult<()> {
        let mut locked = self.shiori.lock()?;
        *locked = None;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_request(&self, h: HGLOBAL, len: &mut usize) -> HGLOBAL {
        match self.request(h, len) {
            Err(e) => {
                error!("{}", e);
                *len = 0;
                ptr::null_mut()
            }
            Ok(rc) => rc,
        }
    }
    pub fn request(&self, h: HGLOBAL, len: &mut usize) -> ShioriResult<HGLOBAL> {
        let g_req = GStr::capture(h, *len);
        let req = g_req.to_utf8_str()?;
        let res = {
            let mut locked = self.shiori.lock()?;
            let shiori = locked.as_mut().ok_or(ErrorKind::NotInitialized)?;
            shiori.request(req)?
        };
        let b_res = res.as_bytes();
        let g_res = GStr::clone_from_slice_nofree(b_res);
        *len = g_res.len();
        Ok(g_res.handle())
    }

    #[allow(dead_code)]
    pub fn raw_shiori3_dll_main(
        &self,
        h_inst: usize,
        ul_reason_for_call: DWORD,
        _lp_reserved: LPVOID,
    ) -> bool {
        match ul_reason_for_call {
            DLL_PROCESS_ATTACH => {
                self.set_h_inst(h_inst);
            }
            DLL_PROCESS_DETACH => {
                self.raw_shiori3_unload();
            }
            _ => {}
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::path::Path;
    use std::path::PathBuf;

    #[derive(Debug)]
    struct TestShiori {
        h_inst: usize,
        load_dir: PathBuf,
    }
    impl Drop for TestShiori {
        fn drop(&mut self) {}
    }
    impl Shiori3 for TestShiori {
        fn new<P: AsRef<Path>>(h_inst: usize, load_dir: P) -> ShioriResult<Self> {
            let shiori = TestShiori {
                h_inst: h_inst,
                load_dir: load_dir.as_ref().to_path_buf(),
            };
            Ok(shiori)
        }
        fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> ShioriResult<Cow<'a, str>> {
            let rc = format!("[{:?}]{} is OK", self, req.into());
            Ok(rc.into())
        }
    }

    #[test]
    fn init_test() {
        {
            ::std::env::set_var("RUST_LOG", "trace");
            let _ = ::env_logger::init();
        }
        let api: RawAPI<TestShiori> = Default::default();
        {
            api.raw_shiori3_dll_main(123, DLL_PROCESS_ATTACH, ptr::null_mut());
            assert_eq!(api.get_h_inst(), 123);
            let locked = api.shiori.lock().unwrap();
            assert!(locked.is_none());
        }
        {
            let load_dir = "load/dir";
            let g_load_dir = GStr::clone_from_str_nofree(load_dir);
            let rc = api.raw_shiori3_load(g_load_dir.handle(), g_load_dir.len());
            assert_eq!(rc, true);
            let mut locked = api.shiori.lock().unwrap();
            assert_eq!(locked.is_some(), true);
            let shiori = locked.as_mut().unwrap();
            assert_eq!(shiori.h_inst, 123);
        }
        {
            let req = "request";
            let g_req = GStr::clone_from_str_nofree(req);
            let mut len = g_req.len();
            let h = api.raw_shiori3_request(g_req.handle(), &mut len);
            let h_res = GStr::capture(h, len);
            let res = h_res.to_utf8_str().unwrap();
            assert_eq!(
                res,
                "[TestShiori { h_inst: 123, load_dir: \"load/dir\" }]request is OK"
            );
        }
        {
            api.raw_shiori3_dll_main(456, DLL_PROCESS_DETACH, ptr::null_mut());
            assert_eq!(api.get_h_inst(), 123);
            let locked = api.shiori.lock().unwrap();
            assert!(locked.is_none());
        }
    }
    #[test]
    fn dir_test() {
        let src_path = file!();
        assert_eq!(src_path, "shiori3_api\\src\\windows.rs");
        /*
        let mut native_path = current_dir().unwrap();
        native_path.pop();
        native_path.push(src_path);
        let native_path_str = native_path.to_str().unwrap();
        assert_eq!(native_path_str, "shiori3_api\\src\\windows.rs");
        */
    }

}
