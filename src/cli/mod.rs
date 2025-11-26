//! CLI module for PPTX tool

pub mod commands;
pub mod parser;

pub use commands::{CreateCommand, FromMarkdownCommand, InfoCommand};
pub use parser::{Parser, Command, CreateArgs, FromMarkdownArgs, InfoArgs};
