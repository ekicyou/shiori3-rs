#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod async_entry;
mod enc;
mod error;
pub mod ext_raw;
mod gstr;
mod parsers;
mod windows;

pub use crate::async_entry as entry;
pub use crate::enc::Encoder;
pub use crate::enc::Encoding;
pub use crate::error::ApiError as ShioriError;
pub use crate::error::ApiResult as ShioriResult;
pub use crate::gstr::GStr;
pub use crate::parsers::req;
