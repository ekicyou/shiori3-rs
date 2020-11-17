#![cfg(any(windows))]

use crate::enc::{Encoder, Encoding};
use crate::error::*;
use std::convert::{AsRef, TryFrom};
use std::ffi::OsString;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str;
use winapi::_core::slice::{from_raw_parts, from_raw_parts_mut};
use winapi::shared::minwindef::{HGLOBAL, UINT};
use winapi::um::winbase::{GlobalAlloc, GlobalFree};
use winapi::vc::vcruntime::size_t;

const GMEM_FIXED: UINT = 0;

// HGLOBAL を 文字列として管理する trait
pub trait GStr<T> {
    fn handle(&self) -> HGLOBAL;
    fn len(&self) -> usize;

    /// 要素を&[u8]として参照します。
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            let p = self.handle() as *mut u8;
            from_raw_parts::<u8>(p, self.len())
        }
    }

    /// (HGLOBAL,len)を取得します。
    fn value(&self) -> (HGLOBAL, usize) {
        (self.handle(), self.len())
    }

    /// 格納データを「ANSI STRING(JP環境ではSJIS)」とみなして、OsStrに変換します。
    /// MultiByteToWideChar()を利用する。
    /// SHIORI::load()文字列の取り出しに利用する。
    fn to_ansi_str(&self) -> ApiResult<OsString> {
        let bytes = self.as_bytes();
        let s = Encoding::ANSI
            .to_string(bytes)
            .map_err(|_| ApiError::EncodeAnsi)?;
        let os_str = OsString::from(s);
        Ok(os_str)
    }

    /// Converts to a string slice.
    /// checks to ensure that the bytes are valid UTF-8, and then does the conversion.
    fn from_utf8(&self) -> ApiResult<&str> {
        let bytes = self.as_bytes();
        Ok(str::from_utf8(bytes)?)
    }

    /// Converts to a string slice
    /// without checking that the string contains valid UTF-8.
    unsafe fn from_utf8_unchecked(&self) -> &str {
        let bytes = self.as_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

// HGLOBAL を 文字列として管理する trait
trait GStrNew {
    fn new(h: HGLOBAL, len: usize) -> Self;
}

trait GStrClone {
    /// &[u8]をHGLOBAL領域にコピーして返す。
    fn clone_from_slice(bytes: &[u8]) -> Self;

    /// HGLOBALを新たに作成し、textをGStrにクローンします。
    #[allow(dead_code)]
    fn clone_from_str<'a, S>(text: S) -> Self
    where
        S: Into<&'a str>;
}

/// HGLOBALを文字列にキャプチャーします。
/// drop時にHGLOBALを解放します。
#[derive(Debug)]
pub struct GStrFree<T> {
    phantom: PhantomData<fn() -> T>,
    h: HGLOBAL,
    len: usize,
}
unsafe impl<T> Send for GStrFree<T> {}

impl<T> fmt::Display for GStrFree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = unsafe { self.from_utf8_unchecked() };
        write!(f, "{}", s)
    }
}

/// HGLOBAL を文字列にキャプチャーします。
/// drop時にHGLOBALを解放しません。
#[derive(Debug)]
pub struct GStrNotFree<T> {
    phantom: PhantomData<fn() -> T>,
    h: HGLOBAL,
    len: usize,
}
unsafe impl<T> Send for GStrNotFree<T> {}

impl<T> fmt::Display for GStrNotFree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = unsafe { self.from_utf8_unchecked() };
        write!(f, "{}", s)
    }
}

pub mod types {
    pub struct GPath;
    pub struct GCowStr;
}

#[allow(dead_code)]
pub type GPath = GStrFree<types::GPath>;
#[allow(dead_code)]
pub type GCowStr = GStrFree<types::GCowStr>;
#[allow(dead_code)]
pub type GPathNotFree = GStrNotFree<types::GPath>;
#[allow(dead_code)]
pub type GCowStrNotFree = GStrNotFree<types::GCowStr>;

impl<T> Drop for GStrFree<T> {
    fn drop(&mut self) {
        unsafe {
            GlobalFree(self.handle());
        }
    }
}

impl<T> GStr<T> for GStrFree<T> {
    fn handle(&self) -> HGLOBAL {
        self.h
    }
    fn len(&self) -> usize {
        self.len
    }
}
impl<T> GStrNew for GStrFree<T> {
    fn new(h: HGLOBAL, len: usize) -> Self {
        Self {
            h,
            len,
            phantom: Default::default(),
        }
    }
}

impl<T> GStr<T> for GStrNotFree<T> {
    fn handle(&self) -> HGLOBAL {
        self.h
    }
    fn len(&self) -> usize {
        self.len
    }
}
impl<T> GStrNew for GStrNotFree<T> {
    fn new(h: HGLOBAL, len: usize) -> Self {
        Self {
            h,
            len,
            phantom: Default::default(),
        }
    }
}

impl<T: GStrNew> GStrClone for T {
    /// &[u8]をHGLOBAL領域にコピーして返す。
    fn clone_from_slice(bytes: &[u8]) -> Self {
        let len = bytes.len();
        let (h, len) = unsafe {
            let h = GlobalAlloc(GMEM_FIXED, len as size_t);
            let p = h as *mut u8;
            let dst = from_raw_parts_mut::<u8>(p, len);
            dst[..].clone_from_slice(bytes);
            (h, len)
        };
        Self::new(h, len)
    }

    /// HGLOBALを新たに作成し、textをGStrにクローンします。
    fn clone_from_str<'a, S>(text: S) -> Self
    where
        S: Into<&'a str>,
    {
        let s = text.into();
        let bytes = s.as_bytes();
        Self::clone_from_slice(bytes)
    }
}

impl AsRef<str> for GCowStr {
    fn as_ref(&self) -> &str {
        unsafe { self.from_utf8_unchecked() }
    }
}

impl Deref for GCowStr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl TryFrom<GPath> for PathBuf {
    type Error = ApiError;
    fn try_from(value: GPath) -> Result<Self, Self::Error> {
        let ansi_str = value.to_ansi_str()?;
        Ok(Into::into(ansi_str))
    }
}

/// HGLOBAL を str として GStr にキャプチャーします。
/// drop時にHGLOBALを開放します。
/// shiori::requestのHGLOBAL受け入れに利用してください。
pub fn capture_str(h: HGLOBAL, len: usize) -> GCowStr {
    GCowStr::new(h, len)
}

/// HGLOBAL を Path として GStr にキャプチャーします。
/// drop時にHGLOBALを開放します。
/// shiori::loadのHGLOBAL受け入れに利用してください。
pub fn capture_path(h: HGLOBAL, len: usize) -> GPath {
    GPath::new(h, len)
}

/// HGLOBALを新たに作成し、textをGStrにクローンします。
/// drop時にHGLOBALを開放しません。
/// shiori応答の作成に利用してください。
#[allow(dead_code)]
pub fn clone_from_str_nofree<'a, S>(text: S) -> GCowStrNotFree
where
    S: Into<&'a str>,
{
    GCowStrNotFree::clone_from_str(text)
}

/// HGLOBALを新たに作成し、pathをGStrにクローンします。
/// drop時にHGLOBALを開放しません。
/// loadリクエストの作成に利用してください。
#[allow(dead_code)]
pub fn clone_from_path_nofree<'a, P>(path: P) -> GPathNotFree
where
    P: Into<&'a Path>,
{
    let text = path.into().to_string_lossy();
    let sjis = Encoding::ANSI.to_bytes(&text).unwrap();
    GPathNotFree::clone_from_slice(&sjis)
}

#[test]
fn gstr_test() {
    {
        let text = "適当なGSTR";
        let src = GCowStrNotFree::clone_from_slice(text.as_bytes());
        assert_eq!(src.from_utf8().unwrap(), text);
        assert_eq!(src.len(), 13);

        let dst = GCowStr::new(src.handle(), src.len());
        assert_eq!(dst.from_utf8().unwrap(), text);
    }
    {
        let text = "適当なGSTR";
        let sjis = Encoding::ANSI.to_bytes(text).unwrap();
        assert_eq!(sjis.len(), 10);
        let src = GPathNotFree::clone_from_slice(&sjis[..]);
        assert_eq!(src.len(), 10);
        let src_osstr = src.to_ansi_str().unwrap();
        assert_eq!(src_osstr.len(), 13);

        let dst = GPath::new(src.handle(), src.len());
        assert_eq!(src_osstr, dst.to_ansi_str().unwrap());

        let src_str = src_osstr.to_str().unwrap();
        assert_eq!(src_str, text);

        let _ = gstr_hresult_test(dst).unwrap();
    }

    fn gstr_hresult_test(gpath: GPath) -> ApiResult<PathBuf> {
        use std::convert::TryInto;
        Ok(gpath.try_into()?)
    }
}
