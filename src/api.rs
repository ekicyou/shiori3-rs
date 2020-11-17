use crate::error::*;
use crate::gstr::{GCowStr, GPath};

pub trait ShioriAPI: Default {
    fn load(&mut self, hinst: usize, load_dir: GPath) -> ApiResult<()>;
    fn unload(&mut self) -> ApiResult<()>;
    fn request(&mut self, req: GCowStr) -> ApiResult<String>;
}
