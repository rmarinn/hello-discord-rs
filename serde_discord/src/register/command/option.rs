use serde::{ser::Error, ser::SerializeStruct, Serialize};

use super::choice::*;
use crate::types::CommandOptionKind;

/// Represents an option for a command, including details such as type, name, description, and other optional fields.
#[non_exhaustive]
pub struct CommandOption {
    kind: CommandOptionKind,
    name: String,
    description: String,
    required: Option<bool>,
    choices: Option<Vec<CommandOptionChoice>>,
    options: Option<Vec<CommandOption>>,
    min_value: Option<CommandOptionChoiceValue>,
    max_value: Option<CommandOptionChoiceValue>,
    min_length: Option<CommandOptionChoiceValue>,
    max_length: Option<CommandOptionChoiceValue>,
    autocomplete: Option<bool>,
}

/// A builder for `CommandOption`, allowing optional and required fields to be set.
impl Serialize for CommandOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("CommandOption", 11)?;
        s.serialize_field("type", &self.kind)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("description", &self.description)?;
        if let Some(required) = self.required {
            s.serialize_field("required", &required)?;
        } else {
            s.skip_field("required")?;
        }
        if let Some(choices) = &self.choices {
            s.serialize_field("choices", &choices)?;
        } else {
            s.skip_field("choices")?;
        }
        if let Some(options) = &self.options {
            s.serialize_field("options", &options)?;
        } else {
            s.skip_field("options")?;
        }
        if let Some(min_value) = &self.min_value {
            match min_value {
                CommandOptionChoiceValue::Int(val) => s.serialize_field("min_value", val)?,
                CommandOptionChoiceValue::Float(val) => s.serialize_field("min_value", val)?,
                CommandOptionChoiceValue::String(_) => {
                    return Err(S::Error::custom("`min_value` should be `Int` or `Float`"))
                }
            }
        } else {
            s.skip_field("min_value")?;
        }
        if let Some(max_value) = &self.max_value {
            match max_value {
                CommandOptionChoiceValue::Int(val) => s.serialize_field("max_value", val)?,
                CommandOptionChoiceValue::Float(val) => s.serialize_field("max_value", val)?,
                CommandOptionChoiceValue::String(_) => {
                    return Err(S::Error::custom("`max_value` should be `Int` or `Float`"))
                }
            }
        } else {
            s.skip_field("max_value")?;
        }
        if let Some(min_length) = &self.min_length {
            match min_length {
                CommandOptionChoiceValue::Int(val) => s.serialize_field("min_length", val)?,
                _ => return Err(S::Error::custom("`min_length` should be `Int`")),
            }
        } else {
            s.skip_field("min_length")?;
        }
        if let Some(max_length) = &self.max_length {
            match max_length {
                CommandOptionChoiceValue::Int(val) => s.serialize_field("max_length", val)?,
                _ => return Err(S::Error::custom("`max_length` should be `Int`")),
            }
        } else {
            s.skip_field("max_length")?;
        }
        if let Some(autocomplete) = self.autocomplete {
            s.serialize_field("autocomplete", &autocomplete)?;
        } else {
            s.skip_field("autocomplete")?;
        }
        s.end()
    }
}

pub struct CommandOptionBuilder {
    kind: Option<CommandOptionKind>,
    name: Option<String>,
    description: Option<String>,
    required: Option<bool>,
    choices: Option<Vec<CommandOptionChoice>>,
    options: Option<Vec<CommandOption>>,
    min_value: Option<CommandOptionChoiceValue>,
    max_value: Option<CommandOptionChoiceValue>,
    min_length: Option<CommandOptionChoiceValue>,
    max_length: Option<CommandOptionChoiceValue>,
    autocomplete: Option<bool>,
}

impl CommandOptionBuilder {
    /// Creates a new instance of the `CommandOptionBuilder`.
    pub fn new() -> Self {
        Self {
            kind: None,
            name: None,
            description: None,
            required: None,
            choices: None,
            options: None,
            min_value: None,
            max_value: None,
            min_length: None,
            max_length: None,
            autocomplete: None,
        }
    }

    /// Sets the type of the command option.
    pub fn kind(mut self, kind: CommandOptionKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Sets the name of the command option.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the description for the command option.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets whether the command option is required.
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Adds a choice to the command option.
    pub fn choice(mut self, choice: CommandOptionChoice) -> Self {
        if let Some(choices) = &mut self.choices {
            choices.push(choice);
        } else {
            self.choices = Some(vec![choice]);
        }
        self
    }

    /// Sets multiple choices for the command option.
    pub fn choices(mut self, choices: Vec<CommandOptionChoice>) -> Self {
        self.choices = Some(choices);
        self
    }

    /// Adds a sub-option to the command option.
    pub fn option(mut self, option: CommandOption) -> Self {
        if let Some(options) = &mut self.options {
            options.push(option);
        } else {
            self.options = Some(vec![option]);
        }
        self
    }

    /// Sets multiple sub-options for the command option.
    pub fn options(mut self, options: Vec<CommandOption>) -> Self {
        self.options = Some(options);
        self
    }

    /// Sets the minimum value for the command option.
    pub fn min_value(mut self, min_value: CommandOptionChoiceValue) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Sets the maximum value for the command option.
    pub fn max_value(mut self, max_value: CommandOptionChoiceValue) -> Self {
        self.max_value = Some(max_value);
        self
    }

    /// Sets the minimum length for the command option (if applicable).
    pub fn min_length(mut self, min_length: CommandOptionChoiceValue) -> Self {
        self.min_length = Some(min_length);
        self
    }

    /// Sets the maximum length for the command option (if applicable).
    pub fn max_length(mut self, max_length: CommandOptionChoiceValue) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Sets whether the command option has autocomplete enabled.
    pub fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.autocomplete = Some(autocomplete);
        self
    }

    /// Builds and returns a `CommandOption` if all required fields are set.
    ///
    /// # Errors
    /// Returns an error if `kind`, `name`, or `description` are not set.
    pub fn build(self) -> Result<CommandOption, Box<dyn std::error::Error>> {
        if self.kind.is_none() {
            return Err("`kind` must be set".into());
        }
        if self.name.is_none() {
            return Err("`name` must be set".into());
        }
        if self.description.is_none() {
            return Err("`description` must be set".into());
        }
        if let Some(min_value) = &self.min_value {
            match min_value {
                CommandOptionChoiceValue::String(_) => {
                    return Err("`min_value` cannot be a string".into())
                }
                _ => (),
            }
        }
        if let Some(value) = &self.max_value {
            match value {
                CommandOptionChoiceValue::String(_) => {
                    return Err("`max_value` cannot be a string".into())
                }
                _ => (),
            }
        }
        if let Some(value) = &self.min_length {
            match value {
                CommandOptionChoiceValue::Int(value) => {
                    if value < &0 {
                        return Err("`min_length` must be at least 0".into());
                    } else if value > &6000 {
                        return Err("`min_length` cannot be greater than 6000".into());
                    }
                }
                _ => return Err("`min_length` must be an integer".into()),
            }
        }
        if let Some(value) = &self.max_length {
            match value {
                CommandOptionChoiceValue::Int(value) => {
                    if value < &1 {
                        return Err("`max_length` must be at least 1".into());
                    } else if value > &6000 {
                        return Err("`max_length` cannot be greater than 6000".into());
                    }
                }
                _ => return Err("`max_length` must be an integer".into()),
            }
        }

        Ok(CommandOption {
            kind: self.kind.unwrap(),
            name: self.name.unwrap(),
            description: self.description.unwrap(),
            required: self.required,
            choices: self.choices,
            options: self.options,
            min_value: self.min_value,
            max_value: self.max_value,
            min_length: self.min_length,
            max_length: self.max_length,
            autocomplete: self.autocomplete,
        })
    }
}
