use serde::{ser::SerializeStruct, Serialize};

use super::MessageComponent;

pub struct ActionRow {
    components: Vec<MessageComponent>,
}

pub struct ActionRowBuilder {
    components: Vec<MessageComponent>,
}

impl ActionRowBuilder {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn components(mut self, components: Vec<MessageComponent>) -> Self {
        self.components = components;
        self
    }

    pub fn component(mut self, component: MessageComponent) -> Self {
        self.components.push(component);
        self
    }

    pub fn build(self) -> ActionRow {
        ActionRow {
            components: self.components,
        }
    }
}

impl Serialize for ActionRow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ActionRow", 2)?;
        s.serialize_field("type", &1)?;
        s.serialize_field("components", &self.components)?;
        s.end()
    }
}
