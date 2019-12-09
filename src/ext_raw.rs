use crate::async_entry as raw;
use crate::error::*;
use crate::ext_api as api;

pub mod dst {
    use crate::ext_api as api;
    use crate::gstr::{GCowStr, GPath};
    pub type LOAD = GPath;
    pub type REQ<'a> = GCowStr<'a>;
    pub type RES = String;
    pub trait LoadExt: api::LoadExt<LOAD> {}
    pub trait UnloadExt: api::UnloadExt {}
    pub trait RequestExt<'a>: api::RequestExt<REQ<'a>, RES> {}
    pub trait EventResponseExt: api::EventResponseExt<RES> {}
}

impl api::LoadExt<dst::LOAD> for raw::Load {
    fn value(self) -> (usize, dst::LOAD) {
        (self.hinst, self.load_dir)
    }
}

impl api::UnloadExt for raw::Unload {
    fn value(self) -> raw::EventResponse<()> {
        self.res
    }
}

impl<'a> api::RequestExt<dst::REQ<'a>, dst::RES> for raw::Request<'a> {
    fn value(self) -> (dst::REQ<'a>, raw::EventResponse<dst::RES>) {
        (self.req, self.res)
    }
}

impl api::EventResponseExt<dst::RES> for raw::EventResponse<dst::RES> {
    fn done(self, item: ApiResult<dst::RES>) -> ApiResult<()> {
        self.send(item)
    }
}

pub use dst::{EventResponseExt, LoadExt, RequestExt, UnloadExt};
impl LoadExt for raw::Load {}
impl UnloadExt for raw::Unload {}
impl<'a> RequestExt<'a> for raw::Request<'a> {}
impl EventResponseExt for raw::EventResponse<dst::RES> {}
