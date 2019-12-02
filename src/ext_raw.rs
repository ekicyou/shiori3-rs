use crate::async_entry as raw;
use crate::error::*;
use crate::gstr::GStr;
use std::path::PathBuf;

pub trait RawLoadExt {
    fn hinst(&self) -> usize;
    fn load_str(&self) -> ApiResult<PathBuf>;
}
impl RawLoadExt for raw::Load {
    fn hinst(&self) -> usize {
        self.hinst
    }
    fn load_str(&self) -> ApiResult<PathBuf> {
        let s = self.load_dir.to_ansi_str()?;
        Ok(PathBuf::from(s))
    }
}

pub trait RawResponseExt<T> {
    fn response(self, item: ApiResult<T>) -> ApiResult<()>;
}
impl RawResponseExt<()> for raw::Unload {
    fn response(self, item: ApiResult<()>) -> ApiResult<()> {
        self.res
            .send(item)
            .map_err(|_| ApiError::EventResponseNotReceived)
    }
}
impl RawResponseExt<GStr> for raw::Request {
    fn response(self, item: ApiResult<GStr>) -> ApiResult<()> {
        self.res
            .send(item)
            .map_err(|_| ApiError::EventResponseNotReceived)
    }
}

pub trait RawRequestExt {
    fn request(&self) -> &GStr;
}
impl RawRequestExt for raw::Request {
    fn request(&self) -> &GStr {
        &self.req
    }
}
