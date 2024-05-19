use crate::commands::*;
use crate::logging::setup_logger;
use clap::{Parser, Subcommand};
use log::{debug, error};
use std::process;

#[derive(Parser)]
#[command(version, author, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print debug information about the hbox environment and configuration
    Info,

    /// List all installed packages and their versions
    List {
        /// Show all versions for a specific package
        name: Option<String>,

        /// Show additional information, like image, volumes, binaries, etc.
        #[arg(long, short)]
        verbose: bool,
    },

    /// Add and install a specific version of a package
    #[command(alias = "install")]
    Add {
        /// Name of the package to install
        name: String,

        /// Version of the package (default: latest)
        #[arg(default_value = "latest")]
        version: String,

        /// Set the added version as the current default version
        #[arg(long, short)]
        set_default: bool,
    },

    /// Remove a specific version of a package
    #[command(alias = "uninstall")]
    Remove {
        /// The name of the package to remove
        name: String,

        /// Version of the package to remove
        version: Option<String>,
    },

    /// Set the current version of a package as the default
    #[command(alias = "set")]
    Use {
        /// Name of the package to set the version for
        name: String,

        /// New version to set as the current default
        version: String,
    },

    /// Run a command from a package
    #[command(disable_help_flag = true)]
    Run {
        /// Name of the package to run
        name: String,

        /// Arguments to pass to the package's command
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        subcommand: Vec<String>,
    },

    /// Configure hbox settings
    #[command(alias = "configure")]
    Config {
        /// Configuration path to set or get
        path: String,

        /// Configuration value to set (omit to get the current value)
        value: Option<String>,
    },
}

pub fn run() {
    if let Err(e) = setup_logger() {
        eprintln!("Could not setup logger: {}", e);
        process::exit(1);
    }

    debug!(
        "hbox {}",
        std::env::args().skip(1).collect::<Vec<String>>().join(" ")
    );

    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Info => show_info(),
        Commands::List { name, verbose } => list_packages(name.as_deref(), *verbose),
        Commands::Add {
            name,
            version,
            set_default,
        } => add_package(name.clone(), version.clone(), *set_default),
        Commands::Remove { name, version } => remove_package(name.clone(), version.clone()),
        Commands::Use { name, version } => use_package_version(name.clone(), version.clone()),
        Commands::Run { name, subcommand } => run_package(name.clone(), subcommand.clone()),
        Commands::Config { path, value } => configure_setting(path.clone(), value.clone()),
    };

    if let Err(e) = result {
        error!("{}", e);
        process::exit(1);
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
