use std::error::Error;

use serde::Serialize;

#[derive(Serialize)]
pub struct SelectMenuOption {
    label: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,
}

pub struct SelectMenuOptionBuilder {
    pub label: Option<String>,
    pub value: Option<String>,
    pub description: Option<String>,
    pub default: Option<String>,
}

impl SelectMenuOptionBuilder {
    pub fn new() -> Self {
        Self {
            label: None,
            value: None,
            description: None,
            default: None,
        }
    }

    pub fn label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn default(mut self, default: String) -> Self {
        self.default = Some(default);
        self
    }

    pub fn build(self) -> Result<SelectMenuOption, Box<dyn Error>> {
        if self.label.is_none() {
            return Err("`label` must be set".into());
        }

        if self.value.is_none() {
            return Err("`value` must be set".into());
        }

        Ok(SelectMenuOption {
            label: self.label.unwrap(),
            value: self.value.unwrap(),
            description: self.description,
            default: self.default,
        })
    }
}

#[non_exhaustive]
pub struct SelectMenu {
    custom_id: String,
    options: Option<Vec<SelectMenuOption>>,
    placeholder: Option<String>,
    disabled: Option<bool>,
}

impl SelectMenu {
    pub fn custom_id(&self) -> &str {
        &self.custom_id
    }

    pub fn options(&self) -> &Option<Vec<SelectMenuOption>> {
        &self.options
    }

    pub fn placeholder(&self) -> &Option<String> {
        &self.placeholder
    }

    pub fn disabled(&self) -> &Option<bool> {
        &self.disabled
    }
}

pub struct SelectMenuBuilder {
    custom_id: Option<String>,
    options: Option<Vec<SelectMenuOption>>,
    placeholder: Option<String>,
    disabled: Option<bool>,
}

impl SelectMenuBuilder {
    pub fn new() -> Self {
        Self {
            custom_id: None,
            options: None,
            placeholder: None,
            disabled: None,
        }
    }

    pub fn custom_id(mut self, custom_id: String) -> Self {
        self.custom_id = Some(custom_id);
        self
    }

    pub fn option(mut self, option: SelectMenuOption) -> Self {
        if let Some(options) = &mut self.options {
            options.push(option);
        } else {
            self.options = Some(vec![option]);
        }
        self
    }

    pub fn options(mut self, options: Vec<SelectMenuOption>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = Some(true);
        self
    }

    pub fn build(self) -> Result<SelectMenu, Box<dyn Error>> {
        if self.custom_id.is_none() {
            return Err("`custom_id` must be set".into());
        }

        Ok(SelectMenu {
            custom_id: self.custom_id.unwrap(),
            options: self.options,
            placeholder: self.placeholder,
            disabled: self.disabled,
        })
    }
}
