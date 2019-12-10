use crate::error::*;
use crate::gstr::*;
use crate::parsers::req::ShioriRequest;

pub trait ShioriRequestExt<'a> {
    fn parse_request(&'a self) -> ApiResult<ShioriRequest<'a>>;
}

impl<'a> ShioriRequestExt<'a> for GCowStr<'a> {
    fn parse_request(&'a self) -> ApiResult<ShioriRequest<'a>> {
        ShioriRequest::parse(self)
    }
}
