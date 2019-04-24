extern crate failure;
extern crate failure_derive;
#[macro_use]
extern crate log;
extern crate pest;
extern crate pest_derive;
extern crate winapi;

mod api;
mod error;
mod hglobal;
mod parsers;

pub use crate::api::Shiori3;
pub use crate::error::Error as ShioriError;
pub use crate::error::ErrorKind as ShioriErrorKind;
pub use crate::error::ShioriResult;
pub use crate::parsers::req::ShioriRequest;
