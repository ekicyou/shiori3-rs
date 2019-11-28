#![cfg(any(windows))]
pub mod enc;
mod windows;

use self::enc::{Encoder, Encoding};
use crate::error::*;
use std::ffi::OsString;
use std::str;
use winapi::_core::slice::{from_raw_parts, from_raw_parts_mut};
use winapi::shared::minwindef::{HGLOBAL, UINT};
use winapi::um::winbase::{GlobalAlloc, GlobalFree};
use winapi::vc::vcruntime::size_t;

const GMEM_FIXED: UINT = 0;

/// HGLOBALを文字列にキャプチャーします。
#[derive(Debug)]
pub struct GStr {
    h: HGLOBAL,
    len: usize,
    has_free: bool,
}
unsafe impl Send for GStr {}

impl Drop for GStr {
    fn drop(&mut self) {
        if !self.has_free {
            return;
        }
        unsafe {
            GlobalFree(self.h);
        }
    }
}

impl GStr {
    /// HGLOBALをGStrにキャプチャーします。
    /// drop時にHGLOBALを開放します。
    /// shiori::load/requestのHGLOBAL受け入れに利用してください。
    pub fn capture(h: HGLOBAL, len: usize) -> GStr {
        GStr {
            h,
            len,
            has_free: true,
        }
    }

    /// &[u8]をHGLOBAL領域にコピーして返す。
    fn clone_from_slice_impl(bytes: &[u8], has_free: bool) -> GStr {
        let len = bytes.len();
        unsafe {
            let h = GlobalAlloc(GMEM_FIXED, len as size_t);
            let p = h as *mut u8;
            let dst = from_raw_parts_mut::<u8>(p, len);
            dst[..].clone_from_slice(bytes);
            GStr { h, len, has_free }
        }
    }

    /// HGLOBALを新たに作成し、&[u8]をGStrにクローンします。
    /// drop時にHGLOBALを開放しません。
    /// shiori応答の作成に利用してください。
    pub fn clone_from_slice_nofree(bytes: &[u8]) -> GStr {
        GStr::clone_from_slice_impl(bytes, false)
    }

    /// HGLOBALを新たに作成し、textをGStrにクローンします。
    /// drop時にHGLOBALを開放します。
    #[allow(dead_code)]
    pub fn clone_from_str<'a, S: Into<&'a str>>(text: S) -> GStr {
        let s = text.into();
        let bytes = s.as_bytes();
        GStr::clone_from_slice_impl(bytes, true)
    }

    /// HGLOBALを新たに作成し、textをGStrにクローンします。
    /// drop時にHGLOBALを開放しません。
    #[allow(dead_code)]
    pub fn clone_from_str_nofree<'a, S: Into<&'a str>>(text: S) -> GStr {
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
    pub fn to_ansi_str(&self) -> MyResult<OsString> {
        let bytes = self.as_bytes();
        let s = Encoding::ANSI
            .to_string(bytes)
            .map_err(|_| MyErrorKind::EncodeAnsi)?;
        let os_str = OsString::from(s);
        Ok(os_str)
    }

    /// 格納データを「UTF-8」とみなして、strに変換する。
    /// SHIORI::request()文字列の取り出しに利用する。
    pub fn to_utf8_str(&self) -> MyResult<&str> {
        let bytes = self.as_bytes();
        Ok(str::from_utf8(bytes)?)
    }
}

#[test]
fn gstr_test() {
    {
        let text = "適当なGSTR";
        let src = GStr::clone_from_slice_nofree(text.as_bytes());
        assert_eq!(src.to_utf8_str().unwrap(), text);
        assert_eq!(src.len(), 13);

        let dst = GStr::capture(src.handle(), src.len());
        assert_eq!(dst.to_utf8_str().unwrap(), text);
    }
    {
        let text = "適当なGSTR";
        let sjis = Encoding::ANSI.to_bytes(text).unwrap();
        assert_eq!(sjis.len(), 10);
        let src = GStr::clone_from_slice_nofree(&sjis[..]);
        assert_eq!(src.len(), 10);
        let src_osstr = src.to_ansi_str().unwrap();
        assert_eq!(src_osstr.len(), 13);

        let dst = GStr::capture(src.handle(), src.len());
        assert_eq!(src_osstr, dst.to_ansi_str().unwrap());

        let src_str = src_osstr.to_str().unwrap();
        assert_eq!(src_str, text);
    }
}
