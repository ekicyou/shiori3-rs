use crate::error::MyErrorKind;
use crate::gstr::GStr;

use failure;
use log::*;
use std::borrow::Cow;
use std::path::Path;
use std::ptr;
use winapi::shared::minwindef::{DWORD, HGLOBAL, LPVOID};

pub trait Shiori3: Sized {
    /// load_dir pathのファイルでSHIORIインスタンスを作成します。
    fn load<P: AsRef<Path>>(
        h_inst: usize,
        load_dir: P,
        load_dir_bytes: &[u8],
    ) -> Result<Self, failure::Error>;

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

impl<T: Shiori3> Shiori3 for Shiori3DI<T> {
    /// load_dir pathのファイルでSHIORIインスタンスを作成します。
    fn load<P: AsRef<Path>>(
        h_inst: usize,
        load_dir: P,
        load_dir_bytes: &[u8],
    ) -> Result<Self, failure::Error> {
        let di = T::load(h_inst, load_dir, load_dir_bytes)?;
        Ok(Shiori3DI { di })
    }

    /// SHIORIリクエストを解釈し、応答を返します。
    fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> Result<Cow<'a, str>, failure::Error> {
        self.di.request(req)
    }
}

/// SHIORI DLL API

#[allow(dead_code)]
#[derive(Default)]
pub struct RawShiori3<T>
where
    T: Shiori3,
{
    h_inst: usize,
    shiori: Option<Shiori3DI<T>>,
}

#[allow(dead_code)]
const DLL_PROCESS_DETACH: DWORD = 0;
#[allow(dead_code)]
const DLL_PROCESS_ATTACH: DWORD = 1;
#[allow(dead_code)]
const DLL_THREAD_ATTACH: DWORD = 2;
#[allow(dead_code)]
const DLL_THREAD_DETACH: DWORD = 3;

impl<T: Shiori3> RawShiori3<T> {
    /// shiori.dll:dllmain
    #[allow(dead_code)]
    pub fn raw_dllmain(
        &mut self,
        h_inst: usize,
        ul_reason_for_call: DWORD,
        _lp_reserved: LPVOID,
    ) -> bool {
        match ul_reason_for_call {
            DLL_PROCESS_ATTACH => {
                self.h_inst = h_inst;
                true
            }
            DLL_PROCESS_DETACH => self.raw_unload(),
            _ => true,
        }
    }

    /// shiori.dll:unload
    #[allow(dead_code)]
    pub fn raw_unload(&mut self) -> bool {
        self.shiori = None;
        true
    }

    /// shiori.dll:load
    #[allow(dead_code)]
    pub fn raw_load(&mut self, hdir: HGLOBAL, len: usize) -> bool {
        self.raw_unload();
        match self.raw_load_impl(hdir, len) {
            Err(e) => {
                error!("[load] {}", e);
                false
            }
            _ => true,
        }
    }
    fn raw_load_impl(&mut self, hdir: HGLOBAL, len: usize) -> Result<(), failure::Error> {
        let gdir = GStr::capture(hdir, len);
        let load_dir = gdir.to_ansi_str()?;
        let load_dir_bytes = gdir.as_bytes();
        let shiori = Shiori3DI::<T>::load(self.h_inst, load_dir, load_dir_bytes)?;
        self.shiori = Some(shiori);
        Ok(())
    }

    /// shiori.dll:request
    #[allow(dead_code)]
    pub fn raw_request(&mut self, hreq: HGLOBAL, len: &mut usize) -> HGLOBAL {
        match self.raw_request_impl(hreq, *len) {
            Err(e) => {
                error!("[request] {}", e);
                *len = 0;
                ptr::null_mut()
            }
            Ok((h, l)) => {
                *len = l;
                h
            }
        }
    }
    fn raw_request_impl(
        &mut self,
        hreq: HGLOBAL,
        len: usize,
    ) -> Result<(HGLOBAL, usize), failure::Error> {
        let greq = GStr::capture(hreq, len);
        let req = greq.to_utf8_str()?;
        let res = {
            let shiori = self.shiori.as_mut().ok_or(MyErrorKind::NotInitialized)?;
            shiori.request(req)?
        };
        let res_bytes = res.as_bytes();
        let gres = GStr::clone_from_slice_nofree(res_bytes);
        Ok(gres.value())
    }
}
