use crate::error::ShioriResult;
use std::borrow::Cow;
use std::fmt::Debug;
use std::path::Path;

pub trait Shiori3: Drop + Sized {
    /// 新しいSHIORIインスタンスを作成します。
    fn load<P: AsRef<Path>>(h_inst: usize, load_dir: P) -> ShioriResult<Self>;

    /// SHIORIリクエストを解釈し、応答を返します。
    fn request<'a, S: Into<&'a str>>(&mut self, req: S) -> ShioriResult<Cow<'a, str>>;
}
