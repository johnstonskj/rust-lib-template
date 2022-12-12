use std::fmt::Display;
use structopt::StructOpt;
use tracing::{info, subscriber::SetGlobalDefaultError};
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter, ParseError},
    FmtSubscriber,
};

// ------------------------------------------------------------------------------------------------
// Command-Line Structure
// ------------------------------------------------------------------------------------------------

const TOOL_NAME: &str = "tool";

#[derive(Debug, StructOpt)]
#[structopt(name = TOOL_NAME)]
struct Cli {
    /// The level of logging to perform, from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Create something new
    New,
    /// Verify an existing thing
    Verify,
}

// ------------------------------------------------------------------------------------------------
// Command-Line Errors
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum TracingSource {
    EnvFilter(ParseError),
    SetGlobal(SetGlobalDefaultError),
}

impl Display for TracingSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EnvFilter(e) => e.to_string(),
                Self::SetGlobal(e) => e.to_string(),
            }
        )
    }
}

impl std::error::Error for TracingSource {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EnvFilter(source) => Some(source),
            Self::SetGlobal(source) => Some(source),
        }
    }
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum ToolError {
    TracingInitFailed(TracingSource),
    WriteToFile,
    VerifyFailed,
}

impl Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TracingInitFailed(e) => e.to_string(),
                Self::WriteToFile => "This thing failed".to_string(),
                Self::VerifyFailed => "This thing failed".to_string(),
            }
        )
    }
}

impl std::error::Error for ToolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TracingInitFailed(source) => Some(source),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Main Function
// ------------------------------------------------------------------------------------------------

fn main() -> Result<(), ToolError> {
    let args = Cli::from_args();

    init_tracing(args.verbose)?;

    match args.cmd {
        Command::New => info!("Will create something new"),
        Command::Verify => info!("Will verify an existing thing"),
    }

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_tracing(verbosity: i8) -> Result<(), ToolError> {
    let log_level = match verbosity {
        0 => LevelFilter::OFF,
        1 => LevelFilter::ERROR,
        2 => LevelFilter::WARN,
        3 => LevelFilter::INFO,
        4 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    let filter = EnvFilter::from_default_env()
        .add_directive(
            format!("{}={}", module_path!(), log_level)
                .parse()
                .map_err(|e| ToolError::TracingInitFailed(TracingSource::EnvFilter(e)))?,
        )
        .add_directive(
            format!("{}={}", TOOL_NAME, log_level)
                .parse()
                .map_err(|e| ToolError::TracingInitFailed(TracingSource::EnvFilter(e)))?,
        );
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| ToolError::TracingInitFailed(TracingSource::SetGlobal(e)))?;
    info!("Log level set to `LevelFilter::{:?}`", log_level);

    Ok(())
}
