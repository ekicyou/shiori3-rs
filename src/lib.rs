#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod async_entry;
mod enc;
mod error;
mod ext_api;
mod ext_raw;
//mod ext_str;
mod gstr;
mod parsers;
mod windows;

pub use crate::enc::Encoder;
pub use crate::enc::Encoding;
pub use crate::error::ApiError as ShioriError;
pub use crate::error::ApiResult as ShioriResult;
pub use crate::gstr::GStr;
pub use crate::parsers::req;

/// raw api entry point
pub mod raw_entry {
    pub use crate::async_entry::*;
    pub use crate::ext_raw::*;
}

/*
/// str api entry point
pub mod str_entry {
    pub use crate::async_entry::*;
    pub use crate::ext_str::*;
}
*/
