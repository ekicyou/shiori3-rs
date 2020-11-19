mod api;
mod enc;
pub mod entry;
mod error;
mod gstr;
mod parsers;
pub mod response;
mod windows;

/// prelude
mod prelude {
    pub use crate::api::ShioriAPI;
    pub use crate::enc::Encoder;
    pub use crate::enc::Encoding;
    pub use crate::entry;
    pub use crate::error::{ApiError, ApiResult};
    pub use crate::gstr::{GCowStr, GPath, GStr};
    pub use crate::parsers::req::{ShioriRequest, ShioriRequestHeader};

    pub use crate::parsers::req::Rule as ShioriRequestRule;
}
pub use prelude::*;

/*



/// str api entry point
pub mod entry {}
*/
