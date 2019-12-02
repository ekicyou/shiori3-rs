use crate::async_entry as raw;
use crate::error::*;
use crate::gstr::GStr;

pub trait LoadExt {
    fn hinst(&self) -> usize;
    fn load_str(&self) -> ApiResult<&GStr>;
}
pub trait ResponseExt<Item> {
    fn response(self, item: ApiResult<Item>) -> ApiResult<()>;
}
pub trait RequestExt {
    fn request(&self) -> &GStr;
}

pub trait RawLoadExt {
    fn raw_hinst(&self) -> usize;
    fn raw_load_str(&self) -> ApiResult<&GStr>;
}
pub trait RawResponseExt<Item> {
    fn raw_response(self, item: ApiResult<Item>) -> ApiResult<()>;
}
pub trait RawRequestExt {
    fn raw_request(&self) -> &GStr;
}

impl<T: RawLoadExt> LoadExt for T {
    fn hinst(&self) -> usize {
        self.raw_hinst()
    }
    fn load_str(&self) -> ApiResult<&GStr> {
        self.raw_load_str()
    }
}
impl<T: RawResponseExt<Item>, Item> ResponseExt<Item> for T {
    fn response(self, item: ApiResult<Item>) -> ApiResult<()> {
        self.raw_response(item)
    }
}
impl<T: RawRequestExt> RequestExt for T {
    fn request(&self) -> &GStr {
        self.raw_request()
    }
}

impl RawLoadExt for raw::Load {
    fn raw_hinst(&self) -> usize {
        self.hinst
    }
    fn raw_load_str(&self) -> ApiResult<&GStr> {
        Ok(&self.load_dir)
    }
}
impl RawResponseExt<()> for raw::Unload {
    fn raw_response(self, item: ApiResult<()>) -> ApiResult<()> {
        self.res
            .send(item)
            .map_err(|_| ApiError::EventResponseNotReceived)
    }
}
impl RawResponseExt<GStr> for raw::Request {
    fn raw_response(self, item: ApiResult<GStr>) -> ApiResult<()> {
        self.res
            .send(item)
            .map_err(|_| ApiError::EventResponseNotReceived)
    }
}
impl RawRequestExt for raw::Request {
    fn raw_request(&self) -> &GStr {
        &self.req
    }
}
