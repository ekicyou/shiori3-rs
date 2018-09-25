#![allow(unused_imports)]
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate pest_derive;
extern crate pest;
#[cfg(any(windows))]
extern crate winapi;

pub mod api;
mod error;
mod hglobal;
mod parsers;

pub use api::Shiori3;
pub use error::Error as ShioriError;
pub use error::ErrorKind as ShioriErrorKind;
pub use error::ShioriResult;
