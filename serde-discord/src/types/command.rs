use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents the type of command in the application.
///
/// This enum defines various command kinds that can be used in a command framework,
/// each represented by a unique unsigned integer value. The command types include:
///
/// - `ChatInput`: A command that can be invoked through a chat input (value 1).
/// - `User`: A command that is related to user interactions (value 2).
/// - `Message`: A command that is tied to message interactions (value 3).
/// - `PrimaryEntryPoint`: A command that serves as the primary entry point for interactions (value 4).
#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum CommandKind {
    /// Command for chat input.
    ChatInput = 1,
    /// Command for user-related interactions.
    User = 2,
    /// Command for message-related interactions.
    Message = 3,
    /// Command serving as the primary entry point.
    PrimaryEntryPoint = 4,
}
