//lib.rs

// region:    --- Modules
mod errors;
pub mod parser;
pub mod runtime;
pub mod tokens;

// endregion: --- Modules

pub type Error = Box<dyn std::error::Error>;

// region:    --- Imports

pub use crate::errors::ParserError;
pub use crate::runtime::environment::Environment;
pub use crate::runtime::values::*;

// endregion: --- Imports
