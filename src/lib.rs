pub mod api;
mod error;
mod hglobal;
mod parsers;

pub use crate::api::Shiori3;
pub use crate::error::Error as ShioriError;
pub use crate::error::ErrorKind as ShioriErrorKind;
pub use crate::error::ShioriResult;
pub use crate::parsers::req::ShioriRequest;
