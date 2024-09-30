mod command;

pub use command::*;
use serde::Deserialize;

/// Represents the data associated with different types of interactions from Discord.
///
/// The `InteractionData` enum currently supports command interactions, but other interaction
/// types (such as message components or modal submits) can be added in the future.
///
/// # Variants
/// - `CommandInteraction`: Contains data specific to command invocations, represented by the `CommandInteractionData` struct.
#[derive(Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum InteractionData {
    /// Represents data associated with a command interaction, such as a slash command or context menu command.
    ///
    /// This variant holds a `CommandInteractionData` struct, which contains detailed information
    /// about the invoked command, its options, and its values.
    CommandInteraction(CommandInteractionData),
    // TODO: addd other variants
}
