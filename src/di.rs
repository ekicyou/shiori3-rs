use crate::prelude::*;
use std::sync::mpsc::Sender;

pub trait ShioriServerAPI {
    fn event_sender(&self) -> ApiResult<Sender<ShioriEvent>>;
}

pub trait ProvideShioriServerAPI {
    type Config: ShioriServerAPI;

    fn provide(&self) -> &Self::Config;
}
