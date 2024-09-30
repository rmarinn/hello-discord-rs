//! The `Interaction` module defines structures and types related to handling incoming interactions from Discord.
//!
//! # Overview
//!
//! The `Interaction` struct is the primary structure that holds information about an interaction event,
//! such as a command invocation or a button click, coming from Discord's API.
//!
//! This module also includes relevant data structures nested under the `data` module, which help
//! in processing the details of each interaction.
//!
//! # Features
//! - Implements `serde::Deserialize` to easily deserialize JSON data from Discord's interaction API.
//! - Provides the `Interaction` struct for representing interaction events.
//! - Contains helper types and data in the `data` module to further facilitate interaction processing.
//!
//! # Example
//!
//! ```rust
//! use my_crate::interaction::Interaction;
//!
//! // Assume `interaction_json` is a JSON string received from Discord's API.
//! let interaction: Interaction = serde_json::from_str(&interaction_json).unwrap();
//!
//! // Now `interaction` can be used to handle the interaction event.
//! ```
//!
//! # Modules
//! - `data`: Contains supporting types for interaction data.
//! - `interaction`: Defines the `Interaction` struct and related functionality.

mod data;
mod interaction;

pub use data::*;
pub use interaction::*;
