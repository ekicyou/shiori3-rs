mod api;
mod enc;
pub mod entry;
mod error;
mod gstr;
mod parsers;
mod windows;

/// prelude
mod prelude {
    pub use crate::api::ShioriAPI;
    pub use crate::enc::Encoder;
    pub use crate::enc::Encoding;
    pub use crate::entry;
    pub use crate::error::{ApiError, ApiResult};
    pub use crate::gstr::{GCowStr, GPath, GStr};
    pub use crate::parsers::req::ShioriRequest;
}
pub use prelude::*;

/*



/// str api entry point
pub mod entry {}
*/
