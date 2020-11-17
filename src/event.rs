use crate::error::*;
use crate::gstr::{GCowStr, GPath};
use crate::parsers::req::ShioriRequest;
use std::sync::mpsc::SyncSender;

/// しおりイベント
pub enum ShioriEvent {
    Request(RequestArgs, SyncSender<Response>),
    Notify(RequestArgs),
    Load(usize, GPath),
    Unload(SyncSender<()>),
}

/// リクエストパラメータ
pub struct RequestArgs {
    req: GCowStr,
    parse: ShioriRequest<'static>,
}
impl RequestArgs {
    pub fn new(req: GCowStr) -> ApiResult<RequestArgs> {
        let parse = ShioriRequest::parse(&req)?;
        unsafe {
            Ok(RequestArgs {
                req: req as _,
                parse: parse,
            })
        }
    }
    fn parse<'a>(&self) -> ShioriRequest<'a> {
        self.parse
    }
}

/// イベントレスポンス
pub struct Response {}
