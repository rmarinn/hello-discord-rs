mod action_row;
mod button;
mod select_menu;
mod text_input;

pub use action_row::*;
pub use button::*;
pub use select_menu::*;
pub use text_input::*;

use serde::{ser::SerializeStruct, Serialize};

#[non_exhaustive]
pub enum MessageComponent {
    ActionRow(ActionRow),
    Button(ButtonComponent),
    StringSelect(SelectMenu),
    TextInput(TextInput),
    UserSelect(SelectMenu),
    RoleSelect(SelectMenu),
    MentionableSelect(SelectMenu),
    ChannelSelect(SelectMenu),
}

impl Serialize for MessageComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            MessageComponent::ActionRow(action_row) => action_row.serialize(serializer),
            MessageComponent::Button(btn) => btn.serialize(serializer),
            MessageComponent::StringSelect(select_menu) => {
                let mut s = serializer.serialize_struct("MessageComponent", 5)?;
                s.serialize_field("type", &3)?;
                s.serialize_field("custom_id", &select_menu.custom_id())?;
                match &select_menu.options() {
                    Some(opts) => s.serialize_field("options", opts)?,
                    None => s.skip_field("options")?,
                }
                match &select_menu.placeholder() {
                    Some(placeholder) => s.serialize_field("placeholder", placeholder)?,
                    None => s.skip_field("placeholder")?,
                }
                match &select_menu.disabled() {
                    Some(disabled) => s.serialize_field("disabled", disabled)?,
                    None => s.skip_field("disabled")?,
                }
                s.end()
            }
            MessageComponent::TextInput(input) => {
                let mut s = serializer.serialize_struct("MessageComponent", 8)?;
                s.serialize_field("type", &4)?;
                s.serialize_field("custom_id", &input.custom_id)?;
                s.serialize_field("label", &input.label)?;
                match &input.min_length {
                    Some(min_length) => s.serialize_field("min_length", min_length)?,
                    None => s.skip_field("min_length")?,
                }
                match &input.max_length {
                    Some(max_length) => s.serialize_field("max_length", max_length)?,
                    None => s.skip_field("max_length")?,
                }
                match &input.required {
                    Some(required) => s.serialize_field("required", required)?,
                    None => s.skip_field("required")?,
                }
                match &input.value {
                    Some(value) => s.serialize_field("value", value)?,
                    None => s.skip_field("value")?,
                }
                match &input.placeholder {
                    Some(placeholder) => s.serialize_field("placeholder", placeholder)?,
                    None => s.skip_field("placeholder")?,
                }
                s.end()
            }
            MessageComponent::UserSelect(select_menu) => {
                let mut s = serializer.serialize_struct("MessageComponent", 5)?;
                s.serialize_field("type", &5)?;
                s.serialize_field("custom_id", &select_menu.custom_id())?;
                match &select_menu.options() {
                    Some(opts) => s.serialize_field("options", opts)?,
                    None => s.skip_field("options")?,
                }
                match &select_menu.placeholder() {
                    Some(placeholder) => s.serialize_field("placeholder", placeholder)?,
                    None => s.skip_field("placeholder")?,
                }
                match &select_menu.disabled() {
                    Some(disabled) => s.serialize_field("disabled", disabled)?,
                    None => s.skip_field("disabled")?,
                }
                s.end()
            }
            MessageComponent::RoleSelect(select_menu) => {
                let mut s = serializer.serialize_struct("MessageComponent", 5)?;
                s.serialize_field("type", &6)?;
                s.serialize_field("custom_id", &select_menu.custom_id())?;
                match &select_menu.options() {
                    Some(opts) => s.serialize_field("options", opts)?,
                    None => s.skip_field("options")?,
                }
                match &select_menu.placeholder() {
                    Some(placeholder) => s.serialize_field("placeholder", placeholder)?,
                    None => s.skip_field("placeholder")?,
                }
                match &select_menu.disabled() {
                    Some(disabled) => s.serialize_field("disabled", disabled)?,
                    None => s.skip_field("disabled")?,
                }
                s.end()
            }
            MessageComponent::MentionableSelect(select_menu) => {
                let mut s = serializer.serialize_struct("MessageComponent", 5)?;
                s.serialize_field("type", &7)?;
                s.serialize_field("custom_id", &select_menu.custom_id())?;
                match &select_menu.options() {
                    Some(opts) => s.serialize_field("options", opts)?,
                    None => s.skip_field("options")?,
                }
                match &select_menu.placeholder() {
                    Some(placeholder) => s.serialize_field("placeholder", placeholder)?,
                    None => s.skip_field("placeholder")?,
                }
                match &select_menu.disabled() {
                    Some(disabled) => s.serialize_field("disabled", disabled)?,
                    None => s.skip_field("disabled")?,
                }
                s.end()
            }
            MessageComponent::ChannelSelect(select_menu) => {
                let mut s = serializer.serialize_struct("MessageComponent", 5)?;
                s.serialize_field("type", &8)?;
                s.serialize_field("custom_id", &select_menu.custom_id())?;
                match &select_menu.options() {
                    Some(opts) => s.serialize_field("options", opts)?,
                    None => s.skip_field("options")?,
                }
                match &select_menu.placeholder() {
                    Some(placeholder) => s.serialize_field("placeholder", placeholder)?,
                    None => s.skip_field("placeholder")?,
                }
                match &select_menu.disabled() {
                    Some(disabled) => s.serialize_field("disabled", disabled)?,
                    None => s.skip_field("disabled")?,
                }
                s.end()
            }
        }
    }
}
