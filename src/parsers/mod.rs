#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate env_logger;

#[macro_use]
extern crate pest_derive;
extern crate pest;

mod req;
mod shiori;

pub use req::{Error, ShioriRequest};
pub use shiori::Rule;
