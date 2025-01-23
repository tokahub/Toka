use serde::Deserialize;
use crate::message::Message;

#[derive(Deserialize)]
pub struct Choice {
    pub index: usize,
    pub message: Message,
    pub finish_reason: String,
}