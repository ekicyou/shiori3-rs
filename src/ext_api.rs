use crate::async_entry as raw;
use crate::error::*;

pub trait LoadExt<T> {
    fn value(self) -> (usize, T);
}
pub trait UnloadExt {
    fn value(self) -> raw::EventResponse<()>;
}
pub trait EventResponseExt<T> {
    fn done(self, item: ApiResult<T>) -> ApiResult<()>;
}
pub trait RequestExt<T> {
    fn value(self) -> (T, raw::EventResponse<T>);
}
