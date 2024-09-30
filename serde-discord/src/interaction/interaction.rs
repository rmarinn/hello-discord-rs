use serde::{de, Deserialize};

use super::data::CommandInteractionData;

/// Represents different types of incoming Discord interactions.
///
/// The `Interaction` enum contains variants for various interaction events
/// that Discord sends, including ping events, command invocations, and component interactions.
///
/// # Variants
/// - `Ping`: Represents a basic "ping" interaction, typically used for confirming bot availability.
/// - `Command`: Represents a slash command invocation with associated data. This variant carries a `CommandInteractionData` value.
/// - `MessageComponent`: Represents an interaction with a message component, such as a button or select menu.
/// - `CommandAutocomplete`: Represents an autocomplete interaction for command input suggestions.
/// - `ModalSubmit`: Represents an interaction when a modal is submitted by a user.
#[derive(Debug)]
pub enum Interaction {
    /// Ping interaction for health checks.
    Ping,

    /// A command interaction that includes command data.
    Command(CommandInteractionData),

    /// Interaction with a message component, such as a button or select menu.
    MessageComponent,

    /// Command autocomplete interaction for providing suggestions to the user.
    CommandAutocomplete,

    /// A modal submit interaction triggered when a user submits a modal.
    ModalSumbit,
}

/// Helper struct to represent the raw data received for an interaction.
///
/// This struct is used internally to deserialize the incoming interaction data
/// before mapping it to an appropriate variant of the `Interaction` enum.
///
/// # Fields
/// - `kind`: A numeric value representing the type of interaction. This field is renamed from `type` in the original JSON.
/// - `data`: Optional interaction data in raw JSON format. Depending on the interaction type, this may be present.
#[derive(Deserialize)]
struct InteractionRaw {
    #[serde(rename = "type")]
    kind: u8,
    data: Option<serde_json::Value>,
}

impl<'de> Deserialize<'de> for Interaction {
    /// Custom deserialization logic for the `Interaction` enum.
    ///
    /// This method takes raw JSON data and maps it to the appropriate variant of `Interaction`
    /// based on the `type` field. If the type corresponds to a command, it also attempts to
    /// deserialize the `data` field into `CommandInteractionData`.
    ///
    /// # Errors
    /// - Returns an error if the interaction type is unknown.
    /// - Returns an error if `CommandInteractionData` fails to deserialize when expected.
    ///
    /// # Example
    /// ```rust
    /// use my_crate::interaction::Interaction;
    ///
    /// let json_str = r#"{
    ///     "type": 2,
    ///     "data": { "name": "example_command" }
    /// }"#;
    ///
    /// let interaction: Interaction = serde_json::from_str(json_str).unwrap();
    /// ```
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = InteractionRaw::deserialize(deserializer)?;

        match raw.kind {
            1 => Ok(Interaction::Ping),
            2 => {
                if let Some(data) = raw.data {
                    let data: CommandInteractionData =
                        serde_json::from_value(data).map_err(|e| {
                            de::Error::custom(format!(
                                "error deserializing `CommandInteractionData`: {:?}",
                                e
                            ))
                        })?;
                    Ok(Interaction::Command(data))
                } else {
                    Err(de::Error::custom(
                        "error deserializing `CommandInteractionData`: no data",
                    ))
                }
            }
            3 => Ok(Interaction::MessageComponent),
            4 => Ok(Interaction::CommandAutocomplete),
            5 => Ok(Interaction::ModalSumbit),
            other => Err(de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "3", "4", "5"],
            )),
        }
    }
}
