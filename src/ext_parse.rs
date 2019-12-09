use crate::async_entry as raw;
use crate::error::*;
use crate::ext_api as api;
use crate::parsers::req;

pub mod dst {
    use crate::error::*;
    use crate::ext_api as api;
    use crate::ext_str::dst as src;
    use crate::gstr::*;
    use crate::parsers::req::ShioriRequest;
    pub use src::{EventResponseExt, LoadExt, UnloadExt, RES};
    use std::convert::AsRef;

    pub struct ReqValues<'a> {
        greq: GCowStr<'a>,
        parse: ShioriRequest<'a>,
    }
    impl<'a> AsRef<ShioriRequest<'a>> for ReqValues<'a> {
        fn as_ref(&self) -> &ShioriRequest<'a> {
            &self.parse
        }
    }
    pub struct REQ<'a>(ApiResult<ReqValues<'a>>);
    impl<'a> REQ<'a> {
        fn parse(greq: GCowStr<'a>) -> ApiResult<ReqValues<'a>> {
            let parse = {
                let s = greq.try_value()?;
                ShioriRequest::parse(s)?
            };
            Ok(ReqValues::<'a> { greq, parse })
        }

        pub fn new(req_gstr: GCowStr<'a>) -> REQ<'a> {
            REQ::<'a>(Self::parse(req_gstr))
        }
    }

    pub trait RequestExt<'a>: api::RequestExt<REQ<'a>, RES> {}
}

impl<'a> api::RequestExt<dst::REQ<'a>, dst::RES> for raw::Request<'a> {
    fn value(self) -> (dst::REQ<'a>, raw::EventResponse<dst::RES>) {
        (dst::REQ::new(self.req), self.res)
    }
}

pub use dst::{EventResponseExt, LoadExt, RequestExt, UnloadExt};
impl<'a> RequestExt<'a> for raw::Request<'a> {}
