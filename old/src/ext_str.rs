use crate::event::*;
use crate::ext_api as api;
use crate::gstr::{GCowStr, GPath};

pub use api::{EventResponseExt, LoadExt, RequestExt, UnloadExt};

impl LoadExt<GPath> for Load {
    fn value(self) -> (usize, GPath) {
        (self.hinst, self.load_dir)
    }
}

impl UnloadExt for Unload {
    fn value(self) -> EventResponse<()> {
        self.res
    }
}

impl RequestExt<GCowStr, String> for Request {
    fn value(self) -> (GCowStr, EventResponse<String>) {
        (self.req, self.res)
    }
}
