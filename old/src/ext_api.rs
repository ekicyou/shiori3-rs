use crate::error::*;
use crate::event::*;

pub trait LoadExt<LOAD> {
    fn value(self) -> (usize, LOAD);
}
pub trait UnloadExt {
    fn value(self) -> EventResponse<()>;
}
pub trait EventResponseExt<RES> {
    fn done(self, item: ApiResult<RES>) -> ApiResult<()>;
}
pub trait RequestExt<REQ, RES> {
    fn value(self) -> (REQ, EventResponse<RES>);
}

impl<T> EventResponseExt<T> for EventResponse<T> {
    fn done(self, item: ApiResult<T>) -> ApiResult<()> {
        self.send(item)
    }
}
