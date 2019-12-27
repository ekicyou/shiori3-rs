use crate::error::*;
use crate::gstr::*;
use futures::channel::mpsc;
use futures::channel::oneshot;
use std::string::String;

/// response sender
pub struct EventResponse<Item>(pub(crate) oneshot::Sender<ApiResult<Item>>);

impl<Item> EventResponse<Item> {
    pub(crate) fn send(self, item: ApiResult<Item>) -> ApiResult<()> {
        self.0
            .send(item)
            .map_err(|_| ApiError::EventResponseNotReceived)
    }
}

/// load event args
pub struct Load {
    pub(crate) hinst: usize,
    pub(crate) load_dir: GPath,
}

/// unload event args
pub struct Unload {
    pub(crate) res: EventResponse<()>,
}

/// request event args
pub struct Request {
    pub(crate) req: GCowStr,
    pub(crate) res: EventResponse<String>,
}

/// SHIORI3 Raw Event
pub enum Event {
    /// load(h_dir: HGLOBAL, len: usize) -> bool
    Load(Load),

    /// unload() -> bool
    Unload(Unload),

    /// request(h: HGLOBAL, len: &mut usize) -> HGLOBAL
    Request(Request),
}

/// SHIORI3 Event Receiver
pub type EventReceiver = mpsc::Receiver<Event>;
pub(crate) type EventSender = mpsc::Sender<Event>;
