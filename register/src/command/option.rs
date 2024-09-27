use serde::{ser::Error, ser::SerializeStruct, Serialize};
use serde_repr::Serialize_repr;

#[derive(Serialize)]
pub enum OptionValue {
    Int(i32),
    Float(f64),
    String(String),
}

#[non_exhaustive]
pub struct CommandOptionChoice {
    name: String,
    value: OptionValue,
}

impl Serialize for CommandOptionChoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Choice", 2)?;
        s.serialize_field("name", &self.name)?;
        match &self.value {
            OptionValue::Int(val) => s.serialize_field("value", val)?,
            OptionValue::Float(val) => s.serialize_field("value", val)?,
            OptionValue::String(val) => {
                if val.len() > 100 {
                    return Err(Error::custom("Value cannot be longer than 100 chars"));
                }
                s.serialize_field("value", val)?;
            }
        };
        s.end()
    }
}

pub struct CommandOptionChoiceBuilder {
    name: Option<String>,
    value: Option<OptionValue>,
}

impl CommandOptionChoiceBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            value: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn value(mut self, value: OptionValue) -> Self {
        self.value = Some(value);
        self
    }

    pub fn build(self) -> Result<CommandOptionChoice, Box<dyn std::error::Error>> {
        if self.name.is_none() {
            return Err("`name` must be set".into());
        }
        if let Some(value) = &self.value {
            match value {
                OptionValue::String(string_val) => {
                    if string_val.len() > 100 {
                        return Err("`value` cannot be longer than 100 chars".into());
                    }
                }
                _ => (),
            }
        } else {
            return Err("`name` must be set".into());
        }

        Ok(CommandOptionChoice {
            name: self.name.unwrap(),
            value: self.value.unwrap(),
        })
    }
}

#[derive(Serialize_repr)]
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

#[non_exhaustive]
pub struct CommandOption {
    kind: CommandOptionKind,
    name: String,
    description: String,
    required: Option<bool>,
    choices: Option<Vec<CommandOptionChoice>>,
    options: Option<Vec<CommandOption>>,
    min_value: Option<OptionValue>,
    max_value: Option<OptionValue>,
    min_length: Option<OptionValue>,
    max_length: Option<OptionValue>,
    autocomplete: Option<bool>,
}

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
            s.serialize_field("min_value", &min_value)?;
        } else {
            s.skip_field("min_value")?;
        }
        if let Some(max_value) = &self.max_value {
            s.serialize_field("max_value", &max_value)?;
        } else {
            s.skip_field("max_value")?;
        }
        if let Some(min_length) = &self.min_length {
            s.serialize_field("min_length", &min_length)?;
        } else {
            s.skip_field("min_length")?;
        }
        if let Some(max_length) = &self.max_length {
            s.serialize_field("max_length", &max_length)?;
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
    min_value: Option<OptionValue>,
    max_value: Option<OptionValue>,
    min_length: Option<OptionValue>,
    max_length: Option<OptionValue>,
    autocomplete: Option<bool>,
}

impl CommandOptionBuilder {
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

    pub fn kind(mut self, kind: CommandOptionKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    pub fn choice(mut self, choice: CommandOptionChoice) -> Self {
        if let Some(choices) = &mut self.choices {
            choices.push(choice);
        } else {
            self.choices = Some(vec![choice]);
        }
        self
    }

    pub fn set_choices(mut self, choices: Vec<CommandOptionChoice>) -> Self {
        self.choices = Some(choices);
        self
    }

    pub fn option(mut self, option: CommandOption) -> Self {
        if let Some(options) = &mut self.options {
            options.push(option);
        } else {
            self.options = Some(vec![option]);
        }
        self
    }

    pub fn set_options(mut self, options: Vec<CommandOption>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn min_value(mut self, min_value: OptionValue) -> Self {
        self.min_value = Some(min_value);
        self
    }

    pub fn max_value(mut self, max_value: OptionValue) -> Self {
        self.max_value = Some(max_value);
        self
    }

    pub fn min_length(mut self, min_length: OptionValue) -> Self {
        self.min_length = Some(min_length);
        self
    }

    pub fn max_length(mut self, max_length: OptionValue) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.autocomplete = Some(autocomplete);
        self
    }

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
                OptionValue::String(_) => return Err("`min_value` cannot be a string".into()),
                _ => (),
            }
        }
        if let Some(value) = &self.max_value {
            match value {
                OptionValue::String(_) => return Err("`max_value` cannot be a string".into()),
                _ => (),
            }
        }
        if let Some(value) = &self.min_length {
            match value {
                OptionValue::Int(value) => {
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
                OptionValue::Int(value) => {
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
