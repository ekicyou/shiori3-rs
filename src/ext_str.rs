use crate::error::*;
use crate::event::*;
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

impl api::LoadExt<dst::LOAD> for Load {
    fn value(self) -> (usize, dst::LOAD) {
        (self.hinst, self.load_dir)
    }
}

impl api::UnloadExt for Unload {
    fn value(self) -> EventResponse<()> {
        self.res
    }
}

impl api::RequestExt<dst::REQ, dst::RES> for Request {
    fn value(self) -> (dst::REQ, EventResponse<dst::RES>) {
        (self.req, self.res)
    }
}

impl api::EventResponseExt<dst::RES> for EventResponse<dst::RES> {
    fn done(self, item: ApiResult<dst::RES>) -> ApiResult<()> {
        self.send(item)
    }
}

pub use dst::{EventResponseExt, LoadExt, RequestExt, UnloadExt};
impl LoadExt for Load {}
impl UnloadExt for Unload {}
impl RequestExt for Request {}
impl EventResponseExt for EventResponse<dst::RES> {}
