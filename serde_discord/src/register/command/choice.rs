use serde::{ser::Error, ser::SerializeStruct, Serialize};

/// Represents the possible values for a command option choice.
/// It can be an integer, a float, or a string.
#[derive(Serialize)]
pub enum ChoiceValue {
    Int(i32),
    Float(f64),
    String(String),
}

/// Represents a choice for a command option, which contains a name and a value.
#[non_exhaustive]
pub struct Choice {
    /// The name of the command option choice.
    name: String,
    /// The value associated with the command option choice.
    value: ChoiceValue,
}

impl Serialize for Choice {
    /// Custom serializer for `CommandOptionChoice` that ensures string values do not exceed 100 characters.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Choice", 2)?;
        s.serialize_field("name", &self.name)?;
        match &self.value {
            ChoiceValue::Int(val) => s.serialize_field("value", val)?,
            ChoiceValue::Float(val) => s.serialize_field("value", val)?,
            ChoiceValue::String(val) => {
                if val.len() > 100 {
                    return Err(Error::custom("Value cannot be longer than 100 chars"));
                }
                s.serialize_field("value", val)?;
            }
        };
        s.end()
    }
}

/// A builder for `CommandOptionChoice`, allowing construction with optional fields.
pub struct CommandOptionChoiceBuilder {
    name: Option<String>,
    value: Option<ChoiceValue>,
}

impl CommandOptionChoiceBuilder {
    /// Creates a new instance of the `CommandOptionChoiceBuilder`.
    pub fn new() -> Self {
        Self {
            name: None,
            value: None,
        }
    }

    /// Sets the name for the command option choice.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the value for the command option choice.
    pub fn value(mut self, value: ChoiceValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Builds and returns a `CommandOptionChoice` if all required fields are set.
    ///
    /// # Errors
    /// Returns an error if `name` or `value` are not set, or if a string value exceeds 100 characters.
    pub fn build(self) -> Result<Choice, Box<dyn std::error::Error>> {
        if self.name.is_none() {
            return Err("`name` must be set".into());
        }
        if let Some(value) = &self.value {
            match value {
                ChoiceValue::String(string_val) => {
                    if string_val.len() > 100 {
                        return Err("`value` cannot be longer than 100 chars".into());
                    }
                }
                _ => (),
            }
        } else {
            return Err("`name` must be set".into());
        }

        Ok(Choice {
            name: self.name.unwrap(),
            value: self.value.unwrap(),
        })
    }
}
