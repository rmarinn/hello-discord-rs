mod choice;
mod option;

use serde::{ser::SerializeStruct, Serialize};
use std::error::Error;

use crate::types::CommandKind;
pub use choice::*;
pub use option::*;

/// Represents a command with a name, type, description, and optional list of command options.
///
/// This struct is used to define the properties of a command in Discord, such as slash commands.
/// It implements `serde::Serialize` to allow it to be serialized for sending to Discord's API.
///
/// # Fields
/// - `name`: The name of the command.
/// - `kind`: The type of command (e.g., slash command, user command, etc.).
/// - `description`: A brief description of the command.
/// - `options`: A list of optional command options (e.g., subcommands or arguments).
pub struct Command {
    name: String,
    kind: CommandKind,
    description: String,
    options: Option<Vec<CommandOption>>,
}

impl Serialize for Command {
    /// Serializes the `Command` struct into a format suitable for Discord's API.
    ///
    /// The fields `name`, `type`, and `description` are always serialized, but the `options` field
    /// is serialized only if it's present. If `options` is `None`, it will be skipped.
    ///
    /// # Errors
    /// Returns an error if the serialization fails.
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

/// A builder for creating a `Command` struct incrementally.
///
/// The `CommandBuilder` allows you to create a `Command` with a flexible API,
/// by specifying each of its fields individually. Once all required fields are set,
/// the `build` method will produce a `Command` instance.
///
/// # Fields
/// - `name`: The name of the command to be created.
/// - `kind`: The type of the command (e.g., slash command).
/// - `description`: A short description of the command.
/// - `options`: An optional list of command options, such as arguments or subcommands.
pub struct CommandBuilder {
    name: Option<String>,
    kind: Option<CommandKind>,
    description: Option<String>,
    options: Option<Vec<CommandOption>>,
}

impl CommandBuilder {
    /// Creates a new `CommandBuilder` with all fields set to `None`.
    ///
    /// Use this method to start building a command from scratch.
    pub fn new() -> Self {
        Self {
            name: None,
            kind: None,
            description: None,
            options: None,
        }
    }

    /// Sets the `name` of the command.
    ///
    /// # Arguments
    /// - `name`: A string that represents the command's name.
    ///
    /// # Example
    /// ```rust
    /// let builder = CommandBuilder::new().name("example_command");
    /// ```
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the `kind` (type) of the command.
    ///
    /// # Arguments
    /// - `kind`: The type of the command (e.g., slash command).
    pub fn kind(mut self, kind: CommandKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Sets the `description` of the command.
    ///
    /// # Arguments
    /// - `description`: A short description explaining what the command does.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a single command option to the command.
    ///
    /// # Arguments
    /// - `option`: A `CommandOption` that represents an option or argument for the command.
    ///
    /// If the options list is already initialized, this method appends the new option to the list.
    /// If it's `None`, the method initializes the list with the provided option.
    pub fn option(mut self, option: CommandOption) -> Self {
        if let Some(options) = &mut self.options {
            options.push(option);
        } else {
            self.options = Some(vec![option]);
        }
        self
    }

    /// Sets the list of command options.
    ///
    /// # Arguments
    /// - `options`: A vector of `CommandOption` structs that represent the arguments or subcommands for the command.
    pub fn options(mut self, options: Vec<CommandOption>) -> Self {
        self.options = Some(options);
        self
    }

    /// Builds and returns a `Command` instance.
    ///
    /// This method checks that the required fields (`name` and `kind`) are set. If they are missing,
    /// it returns an error. Otherwise, it returns the constructed `Command`.
    ///
    /// # Errors
    /// Returns an error if either the `name` or `kind` field is missing.
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
