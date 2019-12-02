use crate::async_entry as raw;
use crate::error::*;
pub use crate::ext_raw::EventResponseExt;
use crate::ext_raw::{RawEventResponseExt, RawEventResponseExt, RawLoadExt, RawRequestExt};
use crate::gstr::GStr;
use std::borrow::Cow;
use std::path::PathBuf;

pub trait LoadExt {
    fn value(self) -> (usize, ApiResult<PathBuf>);
}
pub trait UnloadExt {
    fn value(self) -> raw::EventResponse<()>;
}
pub trait RequestExt {
    fn value<'a>(self) -> (ApiResult<Cow<'a, str>>, raw::EventResponse<GStr>);
}

pub trait StrLoadExt {
    fn str_value(self) -> (usize, ApiResult<PathBuf>);
}
pub trait StrUnloadExt {
    fn str_value(self) -> raw::EventResponse<()>;
}
pub trait StrRequestExt {
    fn str_value<'a>(self) -> (ApiResult<Cow<'a, str>>, raw::EventResponse<GStr>);
}

impl<T: StrLoadExt> LoadExt for T {
    fn value(self) -> (usize, ApiResult<PathBuf>) {
        self.str_value()
    }
}
impl<T: StrUnloadExt> UnloadExt for T {
    fn value(self) -> raw::EventResponse<()> {
        self.str_value()
    }
}
impl<T: StrRequestExt> RequestExt for T {
    fn value<'a>(self) -> (ApiResult<Cow<'a, str>>, raw::EventResponse<GStr>) {
        self.str_value()
    }
}

impl StrLoadExt for raw::Load {
    fn str_value(self) -> (usize, ApiResult<PathBuf>) {
        let (hinst,gstr)=self.raw_value();
        let s = gstr.to_ansi_str();
        (hinst,Ok(s))
    }
}
impl StrUnloadExt for raw::Unload {
    fn str_value(self) -> raw::EventResponse<()> {
       self.raw_value()
    }
}
impl StrRequestExt for raw::Request {
    fn str_value<'a>(self) -> (ApiResult<Cow<'a, str>>, raw::EventResponse<GStr>){
        let (gstr,res)=self.raw_value();

       self.raw_value()
    }
}
impl<'a> From<GStr> for Cow<'a, str> {
    fn from(gstr: GStr) -> Cow<'a, str> {
        
        Cow::Borrowed()
        ApiError::EncodeUtf8
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
    fn str_value<'a>(self) -> (ApiResult<Cow<'a, str>>, raw::EventResponse<GStr>)
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
