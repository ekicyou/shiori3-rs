#![allow(unused_imports)]
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[cfg(any(windows))]
extern crate winapi;
#[macro_use]
extern crate lazy_static;
extern crate rlua;
extern crate shiori3;

mod lua_tests;
mod shiori;
pub mod windows;
