use crate::async_entry as raw;
use crate::error::*;
use crate::ext_api as api;

pub mod dst {
    use crate::error::*;
    use crate::ext_api as api;
    use std::borrow::Cow;
    use std::path::PathBuf;
    pub type ANSI = ApiResult<PathBuf>;
    pub type UTF8<'a> = Cow<'a, str>;

    pub type LOAD = PathBuf;
    pub type REQ<'a> = Cow<'a, str>;
    pub type RES = String;
    pub trait LoadExt: api::LoadExt<LOAD> {}
    pub trait UnloadExt: api::UnloadExt {}
    pub trait RequestExt<'a>: api::RequestExt<REQ<'a>, RES> {}
    pub trait EventResponseExt: api::EventResponseExt<RES> {}
}
use crate::ext_raw::dst as src;

impl<T: src::LoadExt> api::LoadExt<dst::ANSI> for T {
    fn value(self) -> (usize, dst::ANSI) {
        let (hinst, load_dir) = self.value();
        let load_dir = Into::into(load_dir);
        (hinst, load_dir)
    }
}

impl<'a, T: src::RequestExt> api::RequestExt<dst::REQ<'a>, dst::RES> for T {
    fn value(self) -> (dst::REQ<'a>, raw::EventResponse<dst::RES>) {
        let (req, res) = self.value();
        let req = Into::into(req);
        let res = Into::into(req);
        (req, res)
    }
}

impl<T: src::EventResponseExt> api::EventResponseExt<dst::RES> for T {
    fn done(self, item: ApiResult<dst::RES>) -> ApiResult<()> {
        let item = Into::into(item);
        self.send(item)
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
