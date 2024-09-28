use serde::{ser::SerializeStruct, Serialize};

use super::{Autocomplete, Message, Modal};

#[non_exhaustive]
pub enum InteractionResponse {
    Pong,
    Message(Message),
    DeferResponse,
    DeferredUpdateMessage(Message),
    UpdateMessage(Message),
    Autocomplete(Autocomplete),
    Modal(Modal),
}

impl Serialize for InteractionResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("InteractionResponse", 2)?;
        match &self {
            InteractionResponse::Pong => {
                s.serialize_field("type", &1)?;
                s.skip_field("data")?;
            }
            InteractionResponse::Message(msg) => {
                s.serialize_field("type", &4)?;
                s.serialize_field("data", &msg)?;
            }
            InteractionResponse::DeferResponse => {
                s.serialize_field("type", &5)?;
                s.skip_field("data")?;
            }
            InteractionResponse::DeferredUpdateMessage(msg) => {
                s.serialize_field("type", &6)?;
                s.serialize_field("data", &msg)?;
            }
            InteractionResponse::UpdateMessage(msg) => {
                s.serialize_field("type", &7)?;
                s.serialize_field("data", &msg)?;
            }
            InteractionResponse::Autocomplete(autocomplete) => {
                s.serialize_field("type", &8)?;
                s.serialize_field("data", &autocomplete)?;
            }
            InteractionResponse::Modal(modal) => {
                s.serialize_field("type", &9)?;
                s.serialize_field("data", &modal)?;
            }
        };
        s.end()
    }
}
