pub mod cli;
pub mod config;
pub mod core;
pub mod error;
pub mod rules;
pub mod shells;
pub mod types;
pub mod utils;

pub use cli::Cli;
pub use core::run;
pub use error::{ErrorContext, TheFuckError, TheFuckResult};
pub use types::{Command, CommandResult, CorrectedCommand, ParsedCommand, Shell};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
