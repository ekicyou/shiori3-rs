use crate::async_entry as raw;
use crate::error::*;
use crate::gstr::GStr;

pub trait LoadExt {
    fn value(self) -> (usize, GStr);
}
pub trait UnloadExt {
    fn value(self) -> raw::EventResponse<()>;
}
pub trait RequestExt {
    fn value(self) -> (GStr, raw::EventResponse<GStr>);
}
pub trait EventResponseExt<Item> {
    fn done(self, item: ApiResult<Item>) -> ApiResult<()>;
}

pub trait RawLoadExt {
    fn raw_value(self) -> (usize, GStr);
}
pub trait RawUnloadExt {
    fn raw_value(self) -> raw::EventResponse<()>;
}
pub trait RawRequestExt {
    fn raw_value(self) -> (GStr, raw::EventResponse<GStr>);
}

impl<T: RawLoadExt> LoadExt for T {
    fn value(self) -> (usize, GStr) {
        self.raw_value()
    }
}
impl<T: RawUnloadExt> UnloadExt for T {
    fn value(self) -> raw::EventResponse<()> {
        self.raw_value()
    }
}
impl<T: RawRequestExt> RequestExt for T {
    fn value(self) -> (GStr, raw::EventResponse<GStr>) {
        self.raw_value()
    }
}

impl RawLoadExt for raw::Load {
    fn raw_value(self) -> (usize, GStr) {
        (self.hinst, self.load_dir)
    }
}
impl RawUnloadExt for raw::Unload {
    fn raw_value(self) -> raw::EventResponse<()> {
        self.res
    }
}
impl RawRequestExt for raw::Request {
    fn raw_value(self) -> (GStr, raw::EventResponse<GStr>) {
        (self.req, self.res)
    }
}

pub trait RawEventResponseExt<Item> {
    fn raw_done(self, item: ApiResult<Item>) -> ApiResult<()>;
}
impl<T: RawEventResponseExt<Item>, Item> EventResponseExt<Item> for T {
    fn done(self, item: ApiResult<Item>) -> ApiResult<()> {
        self.raw_done(item)
    }
}
impl<Item> RawEventResponseExt<Item> for raw::EventResponse<Item> {
    fn raw_done(self, item: ApiResult<Item>) -> ApiResult<()> {
        self.send(item)
    }
}
