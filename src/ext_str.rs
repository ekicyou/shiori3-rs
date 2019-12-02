use crate::async_entry as raw;
use crate::error::*;
use crate::ext_raw::{RawLoadExt, RawRequestExt, RawResponseExt};
use crate::gstr::GStr;
use std::path::PathBuf;

pub trait LoadExt {
    fn hinst(&self) -> usize;
    fn load_str(&self) -> ApiResult<PathBuf>;
}
pub trait ResponseExt<Item> {
    fn response(self, item: ApiResult<Item>) -> ApiResult<()>;
}
pub trait RequestExt {
    fn request(&self) -> ApiResult<&str>;
}

pub trait StrLoadExt {
    fn str_hinst(&self) -> usize;
    fn str_load_str(&self) -> ApiResult<PathBuf>;
}
pub trait StrResponseExt<Item> {
    fn str_response(self, item: ApiResult<Item>) -> ApiResult<()>;
}
pub trait StrRequestExt {
    fn str_request(&self) -> ApiResult<&str>;
}

impl<T: StrLoadExt> LoadExt for T {
    fn hinst(&self) -> usize {
        self.str_hinst()
    }
    fn load_str(&self) -> ApiResult<PathBuf> {
        self.str_load_str()
    }
}
impl<T: StrResponseExt<Item>, Item> ResponseExt<Item> for T {
    fn response(self, item: ApiResult<Item>) -> ApiResult<()> {
        self.str_response(item)
    }
}
impl<T: StrRequestExt> RequestExt for T {
    fn request(&self) -> ApiResult<&str> {
        self.str_request()
    }
}

impl StrLoadExt for raw::Load {
    fn str_hinst(&self) -> usize {
        self.raw_hinst()
    }
    fn str_load_str(&self) -> ApiResult<PathBuf> {
        let s = self.raw_load_str()?.to_ansi_str()?;
        Ok(PathBuf::from(s))
    }
}
impl StrResponseExt<()> for raw::Unload {
    fn str_response(self, item: ApiResult<()>) -> ApiResult<()> {
        self.raw_response(item)
    }
}
impl<'a, S: Into<&'a str>> StrResponseExt<S> for raw::Request {
    fn str_response(self, item: ApiResult<S>) -> ApiResult<()> {
        let rc = match item {
            Ok(s) => Ok(GStr::clone_from_str(s)),
            Err(e) => Err(e),
        };
        self.raw_response(rc)
    }
}
impl StrRequestExt for raw::Request {
    fn str_request(&self) -> ApiResult<&str> {
        let gstr = self.raw_request();
        gstr.from_utf8()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::async_entry as raw;
    use crate::gstr::GStr;
    use futures::channel::oneshot;
    use futures::executor::LocalPool;

    #[test]
    fn load() {
        let hinst = 12345usize;
        let load_dir = GStr::clone_from_str("TEST");
        let event = raw::Load { hinst, load_dir };
        {
            assert_eq!(event.hinst(), 12345);
            let load_dir = event.load_str().unwrap();
            assert_eq!(load_dir.to_string_lossy(), "TEST");
        }
    }
    #[test]
    fn unload() {
        let rc = LocalPool::new().run_until(async {
            let (res, rx) = oneshot::channel::<ApiResult<()>>();
            let event = raw::Unload { res };

            event.response(Ok(())).unwrap();
            rx.await.unwrap()
        });
        assert_eq!(rc, Ok(()));
    }
    #[test]
    fn request() {
        let g_res = LocalPool::new()
            .run_until(async {
                let req = GStr::clone_from_str("REQUEST");
                let (res, rx) = oneshot::channel::<ApiResult<GStr>>();
                let event = raw::Request { req, res };

                assert_eq!(event.request().unwrap(), "REQUEST");

                event.response(Ok("RESPONSE")).unwrap();
                rx.await.unwrap()
            })
            .unwrap();
        let res = g_res.from_utf8().unwrap();
        assert_eq!(res, "RESPONSE");
    }
}
