#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod api;
pub mod async_raw;
mod enc;
mod error;
mod gstr;
mod parsers;
mod windows;

pub use crate::api::RawShiori3;
pub use crate::api::Shiori3;
pub use crate::enc::Encoder;
pub use crate::enc::Encoding;
pub use crate::error::MyError as ShioriError;
pub use crate::error::MyErrorKind as ShioriErrorKind;
pub use crate::error::MyResult as ShioriResult;
pub use crate::gstr::GStr;
pub use crate::parsers::req;

pub mod executor {
    pub use crate::async_raw as raw;
}
