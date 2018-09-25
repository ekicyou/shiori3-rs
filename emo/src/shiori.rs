use shiori3::*;
use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct EmoShiori {
    h_inst: usize,
    load_dir: PathBuf,
}
impl Drop for EmoShiori {
    fn drop(&mut self) {}
}
impl Shiori3 for EmoShiori {
    fn new<P: AsRef<Path>>(h_inst: usize, load_dir: P) -> ShioriResult<Self> {
        let shiori = EmoShiori {
            h_inst: h_inst,
            load_dir: load_dir.as_ref().to_path_buf(),
        };
        Ok(shiori)
    }
    fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> ShioriResult<Cow<'a, str>> {
        let rc = format!("[{:?}]{} is OK", self, req.into());
        Ok(rc.into())
    }
}
