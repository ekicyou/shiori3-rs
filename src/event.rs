use crate::gstr::GPath;
use std::sync::mpsc::SyncSender;

/// しおりイベント
pub enum ShioriEvent {
    Request(EventArgs, SyncSender<Response>),
    Notify(EventArgs),
    Load(usize, GPath),
    Unload,
}

/// イベントパラメータ
pub struct EventArgs {}

/// イベントレスポンス
pub struct Response {}
