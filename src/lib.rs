mod enc;
mod entry;
mod error;
mod event;
mod gstr;
mod parsers;
mod windows;

/// prelude
mod prelude {
    pub use crate::enc::Encoder;
    pub use crate::enc::Encoding;
    pub use crate::error::{ApiError, ApiResult};
    pub use crate::event::{EventArgs, Response, ShioriEvent};
    pub use crate::gstr::{GCowStr, GPath, GStr};
    pub use crate::parsers::req;
}
pub use prelude::*;

/*



/// str api entry point
pub mod entry {}
*/
