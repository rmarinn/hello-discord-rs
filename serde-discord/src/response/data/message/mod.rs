mod component;

pub use component::*;

use serde::Serialize;

bitflags::bitflags! {
    pub struct MessageFlags: u32 {
        const CROSSPOSTED                      = 1 << 0;
        const IS_CROSSPOST                     = 1 << 1;
        const SUPPRESS_EMBEDS                  = 1 << 2;
        const SOURCE_MESSAGE_DELETED           = 1 << 3;
        const URGENT                           = 1 << 4;
        const HAS_THREAD                       = 1 << 5;
        const EPHEMERAL                        = 1 << 6;
        const LOADING                          = 1 << 7;
        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
        const SUPPRESS_NOTIFICATIONS           = 1 << 12;
        const IS_VOICE_MESSAGE                 = 1 << 13;
    }
}

impl Serialize for MessageFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

#[derive(Serialize)]
#[non_exhaustive]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<MessageComponent>>,
}

pub struct MessageBuilder {
    tts: Option<bool>,
    content: Option<String>,
    flags: Option<MessageFlags>,
    components: Option<Vec<MessageComponent>>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            tts: None,
            content: None,
            flags: None,
            components: None,
        }
    }

    pub fn tts(mut self) -> Self {
        self.tts = Some(true);
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn flags(mut self, flags: MessageFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn component(mut self, component: MessageComponent) -> Self {
        if let Some(components) = &mut self.components {
            components.push(component);
        } else {
            self.components = Some(vec![component]);
        }
        self
    }

    pub fn components(mut self, components: Vec<MessageComponent>) -> Self {
        self.components = Some(components);
        self
    }

    pub fn build(self) -> Message {
        Message {
            tts: self.tts,
            content: self.content,
            flags: self.flags,
            components: self.components,
        }
    }
}
