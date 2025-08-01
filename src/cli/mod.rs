use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "thefuck-rs")]
#[command(about = "A magnificent app which corrects your previous console command")]
#[command(version)]
pub struct Cli {
    /// Print alias for current shell
    #[arg(short, long)]
    #[allow(clippy::type_complexity)]
    pub alias: Option<Option<String>>,

    /// Log shell output to the file
    #[arg(short, long)]
    #[allow(clippy::type_complexity)]
    pub shell_logger: Option<String>,

    /// Enable experimental instant mode
    #[arg(long)]
    pub enable_experimental_instant_mode: bool,

    /// Enable debug output
    #[arg(short, long)]
    pub debug: bool,

    /// Execute fixed command without confirmation
    #[arg(short, long)]
    pub yes: bool,

    /// Repeat on failure
    #[arg(short, long)]
    pub repeat: bool,

    /// Command that should be fixed
    #[arg(trailing_var_arg = true)]
    #[allow(clippy::type_complexity)]
    pub command: Vec<String>,
}

impl Cli {
    pub fn is_alias_request(&self) -> bool {
        self.alias.is_some()
    }

    pub fn is_shell_logger_request(&self) -> bool {
        self.shell_logger.is_some()
    }

    pub fn is_command_fix_request(&self) -> bool {
        !self.command.is_empty() || std::env::var("TF_HISTORY").is_ok()
    }
}
