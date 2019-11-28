mod api;
pub mod async_raw;
mod error;
mod hglobal;
mod parsers;

pub use crate::api::RawShiori3;
pub use crate::api::Shiori3;
pub use crate::error::MyError as ShioriError;
pub use crate::error::MyErrorKind as ShioriErrorKind;
pub use crate::error::MyResult as ShioriResult;
pub use crate::hglobal::enc::Encoder;
pub use crate::hglobal::enc::Encoding;
pub use crate::parsers::req;

pub mod executor {
    pub use crate::async_raw as raw;
}
