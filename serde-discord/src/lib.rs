//! A library for interacting with Discord commands and responses.
//!
//! This library facilitates the registration and management of Discord commands,
//! handling incoming interactions, and generating responses. It provides a set
//! of modules to define types used in interactions, manage command registration,
//! and build appropriate responses for various interaction types.
//!
//! ## Modules
//!
//! - `interaction`: Handles incoming interactions from Discord, allowing for
//!   processing and responding to user inputs.
//! - `register`: Manages the registration of commands, ensuring they are set
//!   up correctly within Discord's system for user access.
//! - `response`: Provides functionality for building and sending responses,
//!   including various interaction response types.
//! - `types`: Defines the types and enums used throughout the library for
//!   representing commands, options, and responses.

/// Module for handling incoming interactions from Discord.
#[cfg(feature = "interaction")]
pub mod interaction;

/// Module for managing the registration of commands with Discord.
#[cfg(feature = "register")]
pub mod register;

/// Module for building and sending responses to user interactions.
#[cfg(feature = "response")]
pub mod response;

/// Module defining types and enums used throughout the library.
#[cfg(feature = "types")]
pub mod types;
