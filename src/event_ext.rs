use crate::async_entry as raw;
use crate::async_entry::{Error, Result};
use crate::gstr::GStr;
use std::path::PathBuf;

pub trait LoadExt {
    fn hinst(&self) -> usize;
    fn load_str(&self) -> Result<PathBuf>;
}
impl LoadExt for raw::Load {
    fn hinst(&self) -> usize {
        self.hinst
    }
    fn load_str(&self) -> Result<PathBuf> {}
}
