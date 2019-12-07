use crate::async_entry as raw;
use crate::error::*;
use crate::ext_api as api;

pub mod dst {
    use crate::ext_api as api;
    pub type Item = crate::gstr::GStr;
    pub trait LoadExt: api::LoadExt<Item> {}
    pub trait UnloadExt: api::UnloadExt {}
    pub trait RequestExt: api::RequestExt<Item> {}
    pub trait EventResponseExt: api::EventResponseExt<Item> {}
}

impl api::LoadExt<dst::Item> for raw::Load {
    fn value(self) -> (usize, dst::Item) {
        (self.hinst, self.load_dir)
    }
}

impl api::UnloadExt for raw::Unload {
    fn value(self) -> raw::EventResponse<()> {
        self.res
    }
}

impl api::RequestExt<dst::Item> for raw::Request {
    fn value(self) -> (dst::Item, raw::EventResponse<dst::Item>) {
        (self.req, self.res)
    }
}

impl api::EventResponseExt<dst::Item> for raw::EventResponse<dst::Item> {
    fn done(self, item: ApiResult<dst::Item>) -> ApiResult<()> {
        self.send(item)
    }
}

impl dst::LoadExt for raw::Load {}
impl dst::UnloadExt for raw::Unload {}
impl dst::RequestExt for raw::Request {}
impl dst::EventResponseExt for raw::EventResponse<dst::Item> {}
