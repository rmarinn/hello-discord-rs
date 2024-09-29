mod command;

use serde::{de, Deserialize};

pub use command::*;

pub type Snowflake = u64;

#[derive(Debug)]
pub enum Interaction {
    Ping,
    Command(CommandInteractionData),
    MessageComponent,
    CommandAutocomplete,
    ModalSumbit,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum InteractionData {
    CommandInteraction(CommandInteractionData),
}

#[derive(Deserialize)]
pub struct InteractionRaw {
    #[serde(rename = "type")]
    kind: u8,
    data: Option<serde_json::Value>,
}

impl<'de> Deserialize<'de> for Interaction {
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
