#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod async_entry;
mod enc;
mod error;
mod event;
mod event;
mod ext_parse;
mod parsers;
mod windows;

pub mod gstr;

pub use prelude::*;
/// prelude
mod prelude {
    pub use crate::enc::Encoder;
    pub use crate::enc::Encoding;
    pub use crate::error::ApiError as ShioriError;
    pub use crate::error::ApiResult as ShioriResult;
    pub use crate::event::*;
    pub use crate::ext_parse::*;
    pub use crate::ext_str::*;
    pub use crate::gstr::{GCowStr, GPath, GStr};
    pub use crate::parsers::req;
}

mod ext_api;
mod ext_str;
/// str api entry point
pub mod entry {
    pub use crate::async_entry::*;
}
