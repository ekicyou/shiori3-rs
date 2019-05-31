use crate::hglobal::GStr;
use crate::winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};
use failure;
use std::borrow::Cow;
use std::default::Default;
use std::ptr;


pub trait Shiori3 {
    /// hinstを設定します。
    fn set_hinst(&mut self, h_inst: usize) -> Result<(), failure::Error>;

    /// load_dir pathのファイルでSHIORIインスタンスを作成します。
    fn load<P: AsRef<Path>>(&mut self, load_dir: P) -> Result<(), failure::Error>;

    /// SHIORIインスタンスを解放します。
    fn unload(&mut self) -> Result<(), failure::Error>;

    /// SHIORIリクエストを解釈し、応答を返します。
    fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> Result<Cow<'a, str>, failure::Error>;
}

#[allow(dead_code)]
struct Shiori3DI<T>
where
    T: Shiori3,
{
    di: T,
}

impl<T: Shiori3> Shiori3DI<T> {
    #[allow(dead_code)]
    fn new(target: T) -> Shiori3DI<T> {
        Shiori3DI { di: target }
    }
}

impl<T: Shiori3 + Default> Default for Shiori3DI<T> {
    #[allow(dead_code)]
    fn default() -> Shiori3DI<T> {
        Shiori3DI::new(T::default())
    }
}

impl<T: Shiori3> Shiori3 for Shiori3DI<T> {
    /// hinstを設定します。
    fn set_hinst(&mut self, hinst: usize) -> Result<(), failure::Error> {
        self.di.set_hinst(hinst)
    }

    /// load_dir pathのファイルでSHIORIインスタンスを作成します。
    fn load<P: AsRef<Path>>(&mut self, load_dir: P) -> Result<(), failure::Error> {
        self.di.load(load_dir)
    }

    /// SHIORIインスタンスを解放します。
    fn unload(&mut self) -> Result<(), failure::Error> {
        self.di.unload()
    }

    /// SHIORIリクエストを解釈し、応答を返します。
    fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> Result<Cow<'a, str>, failure::Error> {
        self.di.request(req)
    }
}

/// SHIORI DLL API
pub trait RawShiori3 {
    /// SHIORI dllmain
    fn raw_dllmain(
        &mut self,
        h_inst: usize,
        ul_reason_for_call: DWORD,
        lp_reserved: LPVOID,
    ) -> bool;

    /// SHIORI Unload
    fn raw_unload(&mut self) -> bool;

    /// SHIORI Load
    fn raw_load(&mut self, hdir: HGLOBAL, len: usize) -> bool;

    /// SHIORI Request
    fn raw_request(&mut self, hreq: HGLOBAL, len: &mut usize) -> HGLOBAL;
}
trait RawShiori3Impl {
    fn raw_load_impl(&mut self, hdir: HGLOBAL, len: usize) -> Result<(), failure::Error>;
    fn raw_request_impl(
        &mut self,
        hreq: HGLOBAL,
        len: usize,
    ) -> Result<(HGLOBAL, usize), failure::Error>;
}

#[allow(dead_code)]
const DLL_PROCESS_DETACH: DWORD = 0;
#[allow(dead_code)]
const DLL_PROCESS_ATTACH: DWORD = 1;
#[allow(dead_code)]
const DLL_THREAD_ATTACH: DWORD = 2;
#[allow(dead_code)]
const DLL_THREAD_DETACH: DWORD = 3;

impl<T: Shiori3> RawShiori3 for T {
    /// shiori.dll:dllmain
    fn raw_dllmain(
        &mut self,
        h_inst: usize,
        ul_reason_for_call: DWORD,
        _lp_reserved: LPVOID,
    ) -> bool {
        match ul_reason_for_call {
            DLL_PROCESS_ATTACH => match self.set_hinst(h_inst) {
                Err(e) => {
                    error!("{}", e);
                    false
                }
                _ => true,
            },
            DLL_PROCESS_DETACH => self.raw_unload(),
            _ => true,
        }
    }

    /// shiori.dll:unload
    fn raw_unload(&mut self) -> bool {
        match self.unload() {
            Err(e) => {
                error!("{}", e);
                false
            }
            _ => true,
        }
    }

    /// shiori.dll:load
    fn raw_load(&mut self, hdir: HGLOBAL, len: usize) -> bool {
        match self.raw_load_impl(hdir, len) {
            Err(e) => {
                error!("{}", e);
                false
            }
            _ => true,
        }
    }

    /// shiori.dll:request
    fn raw_request(&mut self, hreq: HGLOBAL, len: &mut usize) -> HGLOBAL {
        match self.raw_request_impl(hreq, *len) {
            Err(e) => {
                error!("{}", e);
                *len = 0;
                ptr::null_mut()
            }
            Ok((h, l)) => {
                *len = l;
                h
            }
        }
    }
}

impl<T: Shiori3> RawShiori3Impl for T {
    fn raw_load_impl(&mut self, hdir: HGLOBAL, len: usize) -> Result<(), failure::Error> {
        let gdir = GStr::capture(hdir, len);
        let load_dir = gdir.to_ansi_str()?;
        self.load(load_dir)
    }
    fn raw_request_impl(
        &mut self,
        hreq: HGLOBAL,
        len: usize,
    ) -> Result<(HGLOBAL, usize), failure::Error> {
        let greq = GStr::capture(hreq, len);
        let req = greq.to_utf8_str()?;
        let res = self.request(req)?;
        let res_bytes = res.as_bytes();
        let gres = GStr::clone_from_slice_nofree(res_bytes);
        Ok(gres.value())
    }
}
