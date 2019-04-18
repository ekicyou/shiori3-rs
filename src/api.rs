use crate::error::ShioriResult;
use std::borrow::Cow;
use std::fmt::Debug;
use std::path::Path;
use crate::failure;

pub trait Shiori3: Drop + Sized {

    /// hinstを設定します。
    fn set_hinst(&mut self, h_inst: usize) -> Result<(), failure::Error>;

    /// load_dir pathのファイルでSHIORIインスタンスを作成します。
    fn load<P: AsRef<Path>>(&mut self, load_dir: P) -> Result<(), failure::Error>;

    /// SHIORIインスタンスを解放します。
    fn unload(&mut self) -> Result<(), failure::Error>;

    /// SHIORIリクエストを解釈し、応答を返します。
    fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> Result<Cow<'a, str>, failure::Error>;

    /// shiori.dll:dll_main
    fn raw_dll_main(
        &self,
        h_inst: usize,
        ul_reason_for_call: DWORD,
        _lp_reserved: LPVOID,
    ) -> bool {
        let mut unsafe_self = unsafe_as_mut(self);
        match ul_reason_for_call {
            DLL_PROCESS_ATTACH => {
                unsafe_self.set_hinst(h_inst);
            }
            DLL_PROCESS_DETACH => {
                unsafe_self.raw_unload();
            }
            _ => {}
        }
        true
    }

    /// shiori.dll:unload
    fn raw_unload(&self) -> bool {
        let mut unsafe_self = unsafe_as_mut(self);
        match unsafe_self.unload() {
            Err(e) => {
                error!("{}", e);
                false
            }
            _ => true,
        }
    }

    /// shiori.dll:load
    fn raw_load(&self, hdir: HGLOBAL, len: usize) -> bool {
        let mut unsafe_self = unsafe_as_mut(self);
        match unsafe_self.load(hdir, len) {
            Err(e) => {
                error!("{}", e);
                false
            }
            _ => true,
        }
    }

    /// shiori.dll:request
    fn raw_request(&self, h: HGLOBAL, len: &mut usize) -> HGLOBAL {
        let mut unsafe_self = unsafe_as_mut(self);
        match unsafe_self.request(h, len) {
            Err(e) => {
                error!("{}", e);
                *len = 0;
                ptr::null_mut()
            }
            Ok(rc) => rc,
        }
    }
}

struct Shiori3DI<T>
where
    T: Shiori3,
{
    di: T,
}

impl<T: Shiori3> Shiori3DI<T> {
    fn new(T target) -> Shiori3DI<T>
    {
        Shiori3DI{ di:target, }
    }
}

impl<T: Shiori3> Shiori3 for Shiori3DI<T> {
    /// hinstを設定します。
    fn set_hinst(&mut self, hinst: usize) -> Result<(), failure::Error>
    {
        self.di.set_hinst(hinst)
    }

    /// load_dir pathのファイルでSHIORIインスタンスを作成します。
    fn load<P: AsRef<Path>>(&mut self, load_dir: P) -> Result<(), failure::Error>;
    {
        self.di.load(load_dir)
    }

    /// SHIORIインスタンスを解放します。
    fn unload(&mut self) -> Result<(), failure::Error>
    {
        self.di.unload(dir)
    }

    /// SHIORIリクエストを解釈し、応答を返します。
    fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> Result<Cow<'a, str>, failure::Error>
    {
        self.di.request(dir)
    }
}

#[inline]
fn unsafe_as_mut<T>(target: &T) -> &mut T {
    unsafe{
        let p = target as *const T;
        let p_mut = p as *mut T;
        &mut*p_mut
    }
}