#![cfg(any(windows))]

use crate::enc::{Encoder, Encoding};
use crate::error::*;
use std::borrow::Cow;
use std::convert::AsRef;
use std::ffi::OsString;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::str;
use winapi::_core::slice::{from_raw_parts, from_raw_parts_mut};
use winapi::shared::minwindef::{HGLOBAL, UINT};
use winapi::um::winbase::{GlobalAlloc, GlobalFree};
use winapi::vc::vcruntime::size_t;

const GMEM_FIXED: UINT = 0;

/// HGLOBALを文字列にキャプチャーします。
#[derive(Debug)]
pub struct GStr<T> {
    h: HGLOBAL,
    len: usize,
    has_free: bool,
    phantom: PhantomData<fn() -> T>,
}
unsafe impl<T> Send for GStr<T> {}

impl<T> Drop for GStr<T> {
    fn drop(&mut self) {
        if !self.has_free {
            return;
        }
        unsafe {
            GlobalFree(self.h);
        }
    }
}

pub type GPath = GStr<PathBuf>;
pub type GString<'a> = GStr<&'a str>;

/// HGLOBAL を str として GStr にキャプチャーします。
/// drop時にHGLOBALを開放します。
/// shiori::requestのHGLOBAL受け入れに利用してください。
pub fn capture_str<'a>(h: HGLOBAL, len: usize) -> GString<'a> {
    GString::<'a>::capture(h, len)
}

/// HGLOBAL を Path として GStr にキャプチャーします。
/// drop時にHGLOBALを開放します。
/// shiori::loadのHGLOBAL受け入れに利用してください。
pub fn capture_path(h: HGLOBAL, len: usize) -> GPath {
    GPath::capture(h, len)
}

/// HGLOBALを新たに作成し、textをGStrにクローンします。
/// drop時にHGLOBALを開放しません。
/// shiori応答の作成に利用してください。
#[allow(dead_code)]
pub fn clone_from_str_nofree<'a, 'b, S>(text: S) -> GStr<&'a str>
where
    'b: 'a,
    S: Into<&'b str>,
{
    GStr::<&'a str>::clone_from_str_nofree(text)
}

impl<T> GStr<T> {
    fn new(h: HGLOBAL, len: usize, has_free: bool) -> GStr<T> {
        GStr::<T> {
            h,
            len,
            has_free,
            phantom: Default::default(),
        }
    }

    /// HGLOBALをGStrにキャプチャーします。
    /// drop時にHGLOBALを開放します。
    /// shiori::load/requestのHGLOBAL受け入れに利用してください。
    pub fn capture(h: HGLOBAL, len: usize) -> GStr<T> {
        Self::new(h, len, true)
    }

    /// &[u8]をHGLOBAL領域にコピーして返す。
    fn clone_from_slice_impl(bytes: &[u8], has_free: bool) -> GStr<T> {
        let len = bytes.len();
        unsafe {
            let h = GlobalAlloc(GMEM_FIXED, len as size_t);
            let p = h as *mut u8;
            let dst = from_raw_parts_mut::<u8>(p, len);
            dst[..].clone_from_slice(bytes);
            Self::new(h, len, has_free)
        }
    }

    /// HGLOBALを新たに作成し、&[u8]をGStrにクローンします。
    /// drop時にHGLOBALを開放しません。
    /// shiori応答の作成に利用してください。
    pub fn clone_from_slice_nofree(bytes: &[u8]) -> GStr<T> {
        GStr::clone_from_slice_impl(bytes, false)
    }

    /// HGLOBALを新たに作成し、textをGStrにクローンします。
    /// drop時にHGLOBALを開放します。
    #[allow(dead_code)]
    pub fn clone_from_str<'b, S>(text: S) -> GStr<T>
    where
        S: Into<&'b str>,
    {
        let s = text.into();
        let bytes = s.as_bytes();
        GStr::clone_from_slice_impl(bytes, true)
    }

    /// HGLOBALを新たに作成し、textをGStrにクローンします。
    /// drop時にHGLOBALを開放しません。
    /// shiori応答の作成に利用してください。
    #[allow(dead_code)]
    pub fn clone_from_str_nofree<'b, S>(text: S) -> GStr<T>
    where
        S: Into<&'b str>,
    {
        let s = text.into();
        let bytes = s.as_bytes();
        GStr::clone_from_slice_impl(bytes, false)
    }

    /// 要素を&[u8]として参照します。
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let p = self.h as *mut u8;
            from_raw_parts::<u8>(p, self.len)
        }
    }

    /// HGLOBALハンドルを取得します。
    #[allow(dead_code)]
    pub fn handle(&self) -> HGLOBAL {
        self.h
    }

    /// 領域サイズを取得します。
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// (HGLOBAL,len)を取得します。
    #[allow(dead_code)]
    pub fn value(&self) -> (HGLOBAL, usize) {
        (self.h, self.len)
    }

    /// 格納データを「ANSI STRING(JP環境ではSJIS)」とみなして、OsStrに変換します。
    /// MultiByteToWideChar()を利用する。
    /// SHIORI::load()文字列の取り出しに利用する。
    pub fn to_ansi_str(&self) -> ApiResult<OsString> {
        let bytes = self.as_bytes();
        let s = Encoding::ANSI
            .to_string(bytes)
            .map_err(|_| ApiError::EncodeAnsi)?;
        let os_str = OsString::from(s);
        Ok(os_str)
    }

    /// Converts to a string slice.
    /// checks to ensure that the bytes are valid UTF-8, and then does the conversion.
    pub fn from_utf8<'a>(&'a self) -> ApiResult<&'a str> {
        let bytes = self.as_bytes();
        Ok(str::from_utf8(bytes)?)
    }

    /// Converts to a string slice
    /// without checking that the string contains valid UTF-8.
    pub unsafe fn from_utf8_unchecked<'a>(&'a self) -> &'a str {
        let bytes = self.as_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

pub trait TryRefValue<'a, T> {
    fn try_value(&'a self) -> ApiResult<T>;
}
pub trait TryIntoValue<T> {
    fn try_value(self) -> ApiResult<T>;
}

impl<'a> TryRefValue<'a, &'a str> for GString<'a> {
    fn try_value(&'a self) -> ApiResult<&'a str> {
        self.from_utf8()
    }
}

impl TryIntoValue<PathBuf> for GPath {
    fn try_value(self) -> ApiResult<PathBuf> {
        let ansi_str = self.to_ansi_str()?;
        Ok(Into::into(ansi_str))
    }
}

#[test]
fn gstr_test() {
    {
        let text = "適当なGSTR";
        let src = GStr::<&str>::clone_from_slice_nofree(text.as_bytes());
        assert_eq!(src.from_utf8().unwrap(), text);
        assert_eq!(src.len(), 13);

        let dst = GStr::<&str>::capture(src.handle(), src.len());
        assert_eq!(dst.from_utf8().unwrap(), text);
    }
    {
        let text = "適当なGSTR";
        let sjis = Encoding::ANSI.to_bytes(text).unwrap();
        assert_eq!(sjis.len(), 10);
        let src = GStr::<PathBuf>::clone_from_slice_nofree(&sjis[..]);
        assert_eq!(src.len(), 10);
        let src_osstr = src.to_ansi_str().unwrap();
        assert_eq!(src_osstr.len(), 13);

        let dst = GStr::<PathBuf>::capture(src.handle(), src.len());
        assert_eq!(src_osstr, dst.to_ansi_str().unwrap());

        let src_str = src_osstr.to_str().unwrap();
        assert_eq!(src_str, text);
    }
}
