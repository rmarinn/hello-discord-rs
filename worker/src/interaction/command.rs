use serde::{de, Deserialize};
use serde_repr::Deserialize_repr;

use super::Snowflake;

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum CommandKind {
    ChatInput = 1,
    User = 2,
    Message = 3,
    PrimaryEntryPoint = 4,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum CommandOptionKind {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

#[derive(Debug)]
pub enum MultiTypeValue {
    String(String),
    Integer(i64),
    Double(f64),
    Boolean(bool),
}

impl<'de> Deserialize<'de> for MultiTypeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MultiTypeValueVisitor;

        impl<'de> de::Visitor<'de> for MultiTypeValueVisitor {
            type Value = MultiTypeValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string, integer, double, or boolean value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MultiTypeValue::String(value.to_owned()))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MultiTypeValue::Integer(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Cast u64 to i64 (if safe), or handle as double
                if value <= i64::MAX as u64 {
                    Ok(MultiTypeValue::Integer(value as i64))
                } else {
                    Ok(MultiTypeValue::Double(value as f64))
                }
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MultiTypeValue::Double(value))
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MultiTypeValue::Boolean(value))
            }
        }

        deserializer.deserialize_any(MultiTypeValueVisitor)
    }
}

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
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &CommandOptionKind {
        &self.kind
    }

    pub fn value(&self) -> &Option<MultiTypeValue> {
        &self.value
    }

    pub fn options(&self) -> &Option<Vec<CommandInteractionData>> {
        &self.options
    }

    pub fn option(&self, name: &str) -> Option<&CommandInteractionData> {
        if let Some(opts) = self.options() {
            if let Some(opt) = opts.iter().find(|x| x.name() == name) {
                return Some(opt);
            }
        }
        None
    }

    pub fn focused(&self) -> &Option<bool> {
        &self.focused
    }
}

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
    pub fn id(&self) -> &Snowflake {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &CommandKind {
        &self.kind
    }

    pub fn options(&self) -> &Option<Vec<CommandInteractionData>> {
        &self.options
    }

    pub fn option(&self, name: &str) -> Option<&CommandInteractionData> {
        if let Some(opts) = self.options() {
            if let Some(opt) = opts.iter().find(|x| x.name() == name) {
                return Some(opt);
            }
        }
        None
    }

    pub fn guild_id(&self) -> &Option<Snowflake> {
        &self.guild_id
    }

    pub fn target_id(&self) -> &Option<Snowflake> {
        &self.target_id
    }
}
