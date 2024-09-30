use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents the type of command option in the application.
///
/// This enum defines various kinds of command options that can be used in a command framework,
/// each represented by a unique unsigned integer value. The command option types include:
///
/// - `SubCommand`: A command option that represents a sub-command (value 1).
/// - `SubCommandGroup`: A command option that represents a group of sub-commands (value 2).
/// - `String`: A command option that represents a string input (value 3).
/// - `Integer`: A command option that represents an integer input (value 4).
/// - `Boolean`: A command option that represents a boolean value (value 5).
/// - `User`: A command option that represents a user mention (value 6).
/// - `Channel`: A command option that represents a channel mention (value 7).
/// - `Role`: A command option that represents a role mention (value 8).
/// - `Mentionable`: A command option that can mention users, roles, or channels (value 9).
/// - `Number`: A command option that represents a floating-point number (value 10).
/// - `Attachment`: A command option that represents a file attachment (value 11).
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum CommandOptionKind {
    /// Represents a sub-command.
    SubCommand = 1,
    /// Represents a group of sub-commands.
    SubCommandGroup = 2,
    /// Represents a string input.
    String = 3,
    /// Represents an integer input.
    Integer = 4,
    /// Represents a boolean value.
    Boolean = 5,
    /// Represents a user mention.
    User = 6,
    /// Represents a channel mention.
    Channel = 7,
    /// Represents a role mention.
    Role = 8,
    /// Represents a mentionable entity (users, roles, channels).
    Mentionable = 9,
    /// Represents a floating-point number.
    Number = 10,
    /// Represents a file attachment.
    Attachment = 11,
}
