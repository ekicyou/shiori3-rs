use crate::async_entry as raw;
use crate::error::*;

pub trait LoadExt<LOAD> {
    fn value(self) -> (usize, LOAD);
}
pub trait UnloadExt {
    fn value(self) -> raw::EventResponse<()>;
}
pub trait EventResponseExt<RES> {
    fn done(self, item: ApiResult<RES>) -> ApiResult<()>;
}
pub trait RequestExt<REQ, RES> {
    fn value(self) -> (REQ, raw::EventResponse<RES>);
}
