use crate::async_entry as raw;
use crate::error::*;
use crate::ext_api as api;

pub mod dst {
    use crate::ext_api as api;
    use crate::gstr::{GCowStr, GPath};
    pub type LOAD = GPath;
    pub type REQ = GCowStr;
    pub type RES = String;
    pub trait LoadExt: api::LoadExt<LOAD> {}
    pub trait UnloadExt: api::UnloadExt {}
    pub trait RequestExt: api::RequestExt<REQ, RES> {}
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

impl api::RequestExt<dst::REQ, dst::RES> for raw::Request {
    fn value(self) -> (dst::REQ, raw::EventResponse<dst::RES>) {
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
impl RequestExt for raw::Request {}
impl EventResponseExt for raw::EventResponse<dst::RES> {}
