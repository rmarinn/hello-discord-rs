use serde::{ser::SerializeStruct, Serialize};
use serde_repr::Serialize_repr;
use std::error::Error;

use super::CommandOption;

pub struct Command {
    name: String,
    kind: CommandKind,
    description: String,
    options: Option<Vec<CommandOption>>,
}

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Command", 4)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("type", &self.kind)?;
        s.serialize_field("description", &self.description)?;
        if let Some(options) = &self.options {
            s.serialize_field("options", &options)?;
        } else {
            s.skip_field("options")?;
        }
        s.end()
    }
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum CommandKind {
    ChatInput = 1,
    User = 2,
    Message = 3,
    PrimaryEntryPoint = 4,
}

pub struct CommandBuilder {
    name: Option<String>,
    kind: Option<CommandKind>,
    description: Option<String>,
    options: Option<Vec<CommandOption>>,
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            kind: None,
            description: None,
            options: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn kind(mut self, kind: CommandKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn option(mut self, option: CommandOption) -> Self {
        if let Some(options) = &mut self.options {
            options.push(option);
        } else {
            self.options = Some(vec![option]);
        }
        self
    }

    pub fn set_options(mut self, options: Vec<CommandOption>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn build(self) -> Result<Command, Box<dyn Error>> {
        if self.name.is_none() {
            return Err("`name` must be set".into());
        }
        if self.kind.is_none() {
            return Err("`kind` must be set".into());
        }

        Ok(Command {
            name: self.name.unwrap(),
            kind: self.kind.unwrap(),
            description: self.description.unwrap_or_default(),
            options: self.options,
        })
    }
}
