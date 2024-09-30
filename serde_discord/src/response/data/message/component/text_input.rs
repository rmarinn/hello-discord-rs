use std::error::Error;

use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum TextInputStyle {
    Short = 1,
    Paragraph = 2,
}

#[derive(Serialize)]
pub struct TextInput {
    pub custom_id: String,
    pub style: TextInputStyle,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

pub struct TextInputBuilder {
    pub custom_id: Option<String>,
    pub style: Option<TextInputStyle>,
    pub label: Option<String>,
    pub min_length: Option<u16>,
    pub max_length: Option<u16>,
    pub required: Option<bool>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
}

impl TextInputBuilder {
    pub fn new() -> Self {
        Self {
            custom_id: None,
            style: None,
            label: None,
            min_length: None,
            max_length: None,
            required: None,
            value: None,
            placeholder: None,
        }
    }

    pub fn custom_id(mut self, custom_id: String) -> Self {
        self.custom_id = Some(custom_id);
        self
    }

    pub fn style(mut self, style: TextInputStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn min_length(mut self, min_length: u16) -> Self {
        self.min_length = Some(min_length);
        self
    }

    pub fn max_length(mut self, max_length: u16) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn required(mut self) -> Self {
        self.required = Some(true);
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn build(self) -> Result<TextInput, Box<dyn Error>> {
        if self.custom_id.is_none() {
            return Err("`custom_id` must be set".into());
        }
        if self.style.is_none() {
            return Err("`style` must be set".into());
        }
        if let Some(label) = &self.label {
            if label.len() > 45 {
                return Err("`label` cannot be longet than 45 chars".into());
            }
        }
        if let Some(min_length) = self.min_length {
            if min_length > 4000 {
                return Err("`min_length` must be between 0 and 4000 chars".into());
            }
        }
        if let Some(max_length) = self.max_length {
            if max_length < 1 || max_length > 4000 {
                return Err("`min_length` must be between 1 and 4000 chars".into());
            }
        }
        Ok(TextInput {
            custom_id: self.custom_id.unwrap(),
            style: self.style.unwrap(),
            label: self.label.unwrap(),
            min_length: self.min_length,
            max_length: self.max_length,
            required: self.required,
            value: self.value,
            placeholder: self.placeholder,
        })
    }
}
