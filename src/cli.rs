use crate::commands::*;
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(version, author, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print debug information
    Info,

    /// List all installed packages and their versions
    List {
        /// Show all versions for a given package
        name: Option<String>,
    },

    /// Add a specific version of a package
    #[command(alias = "install")]
    Add {
        /// Name of the package to install
        name: String,

        /// Version of the package (default: latest)
        #[arg(default_value = "latest")]
        version: String,

        /// Set the added version as the current version
        #[arg(long, short)]
        set_default: bool,
    },

    /// Remove a package
    #[command(alias = "uninstall")]
    Remove {
        /// The name of the package to remove
        name: String,

        /// Version of the package
        version: Option<String>,
    },

    /// Set current version of a package
    #[command(alias = "set")]
    Use {
        /// Name of the package to set the version of
        name: String,

        /// New version to set as current
        version: String,
    },

    /// Run the package
    #[command(disable_help_flag = true)]
    Run {
        /// Name of the package to run
        name: String,

        /// Arguments to pass to the package
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        subcommand: Vec<String>,
    },
}

pub fn run() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Info => show_info(),
        Commands::List { name } => list_packages(name.as_deref()),
        Commands::Add {
            name,
            version,
            set_default,
        } => add_package(name.clone(), version.clone(), *set_default),
        Commands::Remove { name, version } => remove_package(name.clone(), version.clone()),
        Commands::Use { name, version } => set_package_version(name.clone(), version.clone()),
        Commands::Run { name, subcommand } => run_package(name.clone(), subcommand.clone()),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        process::exit(1);
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
