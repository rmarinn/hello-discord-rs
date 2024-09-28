use std::error::Error;

use serde::{ser::SerializeStruct, Serialize};
use serde_repr::Serialize_repr;

#[derive(Serialize_repr)]
#[repr(u8)]
#[allow(dead_code)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
    Premium = 6,
}

#[non_exhaustive]
pub struct ButtonComponent {
    style: ButtonStyle,
    label: Option<String>,
    custom_id: Option<String>,
    url: Option<String>,
    disabled: Option<bool>,
}

impl Serialize for ButtonComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ButtonMessageComponent", 5)?;
        s.serialize_field("type", &5)?;
        s.serialize_field("style", &self.style)?;
        match &self.label {
            Some(label) => s.serialize_field("label", label)?,
            None => s.skip_field("label")?,
        }
        match &self.custom_id {
            Some(custom_id) => s.serialize_field("custom_id", custom_id)?,
            None => s.skip_field("custom_id")?,
        }
        match &self.url {
            Some(url) => s.serialize_field("url", url)?,
            None => s.skip_field("url")?,
        }
        match &self.disabled {
            Some(disabled) => s.serialize_field("url", disabled)?,
            None => s.skip_field("disabled")?,
        }
        s.end()
    }
}

pub struct ButtonComponentBuilder {
    style: Option<ButtonStyle>,
    label: Option<String>,
    custom_id: Option<String>,
    url: Option<String>,
    disabled: Option<bool>,
}

impl ButtonComponentBuilder {
    pub fn new() -> Self {
        Self {
            style: None,
            label: None,
            custom_id: None,
            url: None,
            disabled: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn custom_id(mut self, custom_id: String) -> Self {
        self.custom_id = Some(custom_id);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = Some(true);
        self
    }

    pub fn build(self) -> Result<ButtonComponent, Box<dyn Error>> {
        if self.style.is_none() {
            return Err("`style` must be set".into());
        }

        Ok(ButtonComponent {
            style: self.style.unwrap(),
            label: self.label,
            custom_id: self.custom_id,
            url: self.url,
            disabled: self.disabled,
        })
    }
}
