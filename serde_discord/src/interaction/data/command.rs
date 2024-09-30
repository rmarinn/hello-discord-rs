use serde::Deserialize;

use crate::types::{CommandKind, CommandOptionKind, MultiTypeValue, Snowflake};

/// Represents data associated with a specific command interaction.
///
/// This struct is used to hold information about a command invocation, including the command's name, type,
/// potential value, and any nested options or subcommands.
///
/// # Fields
/// - `name`: The name of the invoked command or subcommand.
/// - `kind`: The kind of command option, such as a string, integer, or boolean.
/// - `value`: The optional value associated with the command option, if applicable.
/// - `options`: A list of subcommand or option data if the command has nested options.
/// - `focused`: Indicates whether this option is currently focused by the user, often used in autocomplete.
#[derive(Deserialize, Debug)]
pub struct CommandInteractionData {
    name: String,
    #[serde(rename = "type")]
    kind: CommandOptionKind,
    value: Option<MultiTypeValue>,
    options: Option<Vec<CommandInteractionData>>,
    focused: Option<bool>,
}

impl CommandInteractionData {
    /// Returns the name of the command or subcommand.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the kind of the command option.
    pub fn kind(&self) -> &CommandOptionKind {
        &self.kind
    }

    /// Returns an optional value associated with the command, if present.
    pub fn value(&self) -> &Option<MultiTypeValue> {
        &self.value
    }

    /// Returns any nested options for this command, such as subcommands or option lists.
    pub fn options(&self) -> &Option<Vec<CommandInteractionData>> {
        &self.options
    }

    /// Returns a specific option by name, if it exists among the command's options.
    ///
    /// This method searches the `options` list for an option with the specified name.
    /// # Example
    /// ```rust
    /// let option = command_data.option("subcommand_name");
    /// if let Some(subcommand) = option {
    ///     // Handle the subcommand
    /// }
    /// ```
    pub fn option(&self, name: &str) -> Option<&CommandInteractionData> {
        if let Some(opts) = self.options() {
            if let Some(opt) = opts.iter().find(|x| x.name() == name) {
                return Some(opt);
            }
        }
        None
    }

    /// Returns whether this option is focused, typically used in autocomplete interactions.
    pub fn focused(&self) -> &Option<bool> {
        &self.focused
    }
}

/// Represents the overall data structure for a command interaction.
///
/// This struct holds the information for a command interaction, including its ID, name, and type,
/// as well as any associated options or the target guild and target entity.
///
/// # Fields
/// - `id`: A unique identifier for the command.
/// - `name`: The name of the command being invoked.
/// - `kind`: The type of command, such as slash command, message command, etc.
/// - `options`: Optional list of options or subcommands for the command.
/// - `guild_id`: Optional guild ID where the command was invoked.
/// - `target_id`: Optional target ID if the command involves a specific target (e.g., a user or message).
#[derive(Deserialize)]
#[non_exhaustive]
pub struct CommandData {
    id: Snowflake,
    name: String,
    #[serde(rename = "type")]
    kind: CommandKind,
    options: Option<Vec<CommandInteractionData>>,
    guild_id: Option<Snowflake>,
    target_id: Option<Snowflake>,
}

impl CommandData {
    /// Returns the unique ID of the command.
    pub fn id(&self) -> &Snowflake {
        &self.id
    }

    /// Returns the name of the command.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the command.
    pub fn kind(&self) -> &CommandKind {
        &self.kind
    }

    /// Returns any options or subcommands associated with the command, if present.
    pub fn options(&self) -> &Option<Vec<CommandInteractionData>> {
        &self.options
    }

    /// Returns a specific option by name, if it exists among the command's options.
    ///
    /// This method searches the `options` list for an option with the specified name.
    /// # Example
    /// ```rust
    /// let option = command_data.option("option_name");
    /// if let Some(option) = option {
    ///     // Handle the option
    /// }
    /// ```
    pub fn option(&self, name: &str) -> Option<&CommandInteractionData> {
        if let Some(opts) = self.options() {
            if let Some(opt) = opts.iter().find(|x| x.name() == name) {
                return Some(opt);
            }
        }
        None
    }

    /// Returns the ID of the guild where the command was invoked, if applicable.
    pub fn guild_id(&self) -> &Option<Snowflake> {
        &self.guild_id
    }

    /// Returns the ID of the target (user or message) involved in the command, if applicable.
    pub fn target_id(&self) -> &Option<Snowflake> {
        &self.target_id
    }
}
