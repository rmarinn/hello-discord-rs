use serde::{ser::SerializeStruct, Serialize};

use super::data::{Autocomplete, Message, Modal};

/// Represents different types of interaction responses.
/// The interaction response can range from simple acknowledgments like Pong to complex messages, modals, or autocomplete suggestions.
#[non_exhaustive]
pub enum InteractionResponse {
    /// A Pong response, typically used to acknowledge the interaction.
    Pong,
    /// A message response with embedded content or components.
    Message(Message),
    /// A deferred response, indicating that the bot is thinking, without sending any immediate content.
    DeferResponse,
    /// A deferred message update, which may contain content or components that will be sent later.
    DeferredUpdateMessage(Message),
    /// An updated message response, allowing an existing message to be modified.
    UpdateMessage(Message),
    /// A response that provides autocomplete suggestions.
    Autocomplete(Autocomplete),
    /// A modal response, typically used to present a form to the user.
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
