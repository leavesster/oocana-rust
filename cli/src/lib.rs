mod commands;

use std::path::PathBuf;
use clap::{AppSettings, Parser, IntoApp, Subcommand};
use clap_complete::{generate, shells::{Bash, Fish, Zsh}};

use utils::app_config::AppConfig;
use utils::error::Result;
use utils::types::LogLevel;

#[derive(Parser, Debug)]
#[clap(
    name = "vocana",
    author,
    about,
    long_about = "Vocana CLI",
    version
)]
#[clap(setting = AppSettings::SubcommandRequired)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Cli {
    /// Set a custom config file
    #[clap(short, long,parse(from_os_str), value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Set a custom config file
    #[clap(name="debug", short, long="debug", value_name = "DEBUG")]
    pub debug: Option<bool>,

    /// Set Log Level 
    #[clap(name="log_level", short, long="log-level", value_name = "LOG_LEVEL")]
    pub log_level: Option<LogLevel>,

    /// Subcommands
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(
        name = "run",
        about = "Run a Vocana Flow",
        long_about = None, 
    )]
    Run {
        #[clap(help = "Path to the Vocana Block Manifest file or a directory with flow.oo.yaml.")]
        block: String,
        #[clap(help = "MQTT Broker Address.", long)]
        broker: Option<String>,
        #[clap(help = "Paths to search for blocks. Fallback to the directory of current flow block.", long)]
        block_search_paths: Option<String>,
        #[clap(help = "Optional id to mark this execution session.", long)]
        session: Option<String>,
        #[clap(help = "Enable reporter.", long)]
        reporter: bool,
        #[clap(help = "Stop the flow after the node is finished.", long)]
        node: Option<String>,
    },
    #[clap(
        name = "completion",
        about = "Generate completion scripts",
        long_about = None,
        )]
        Completion {
            #[clap(subcommand)]
            subcommand: CompletionSubcommand,
        },
    #[clap(
        name = "config",
        about = "Show Configuration",
        long_about = None,
    )]
    Config,
}

#[derive(Subcommand, PartialEq, Debug)]
enum CompletionSubcommand {
    #[clap(about = "generate the autocompletion script for bash")]
    Bash,
    #[clap(about = "generate the autocompletion script for zsh")]
    Zsh,
    #[clap(about = "generate the autocompletion script for fish")]
    Fish,
}

pub fn cli_match() -> Result<()> {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Merge clap config file if the value is set
    AppConfig::merge_config(cli.config.as_deref())?;

    let app = Cli::into_app();
    
    AppConfig::merge_args(app)?;

    // Execute the subcommand
    match &cli.command {
        Commands::Run { block, broker, block_search_paths, session, reporter, node } => {
            commands::run(block, broker.to_owned(), block_search_paths.to_owned(), session.to_owned(), reporter.to_owned(), node.to_owned())?
        },
        Commands::Completion {subcommand} => {
            let mut app = Cli::into_app();
            match subcommand {
                CompletionSubcommand::Bash => {
                    generate(Bash, &mut app, "vocana", &mut std::io::stdout());
                }
                CompletionSubcommand::Zsh => {
                    generate(Zsh, &mut app, "vocana", &mut std::io::stdout());
                }
                CompletionSubcommand::Fish => {
                    generate(Fish, &mut app, "vocana", &mut std::io::stdout());
                }
            }
        }
        Commands::Config => commands::config()?,
    }

    Ok(())
}
