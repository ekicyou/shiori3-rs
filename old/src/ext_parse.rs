use crate::error::*;
use crate::gstr::*;
use crate::parsers::req::ShioriRequest;

pub trait ShioriRequestExt {
    /// SHIORI REQUEST 3 として解析します。
    fn parse_request(&self) -> ApiResult<ShioriRequest>;
}

impl ShioriRequestExt for GCowStr {
    /// SHIORI REQUEST 3 として解析します。
    fn parse_request(&self) -> ApiResult<ShioriRequest> {
        ShioriRequest::parse(self)
    }
}
