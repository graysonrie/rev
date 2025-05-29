use std::io::stdin;

use clap::Parser;
mod cmds;
mod external_cmds;
mod state;
mod utils;

/// Revit CLI - A command line tool for managing Revit add-in projects
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Build the project
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Build the project
    ///
    /// Attempts to build using MSBuild first, falling back to dotnet build if MSBuild is not available.
    /// The project must contain a valid .csproj file.
    Build,

    /// Export the add-in to Revit's add-in directory
    ///
    /// Creates necessary add-in files, builds the project, and copies all required files
    /// to the appropriate Revit add-in directory. Handles both standard add-ins and those
    /// with web-based UIs (Next.js).
    Export,

    /// Display the currently configured Revit version
    ///
    /// Shows which version of Revit (year) the CLI is currently configured to work with.
    /// This affects where add-ins are exported to.
    RevitVersion,

    /// Change the target Revit version
    ///
    /// Updates which version of Revit (year) to target when exporting add-ins.
    /// Valid versions are from 2019 to 2025.
    ChangeRevitVersion,

    /// Locate the project DLL
    ///
    /// Prints out the full path to the project DLL
    Locate,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Build => cmds::build::execute(),
        Commands::Export => cmds::export::execute(),
        Commands::RevitVersion => {
            let revit_version = ensure_revit_version_is_set();
            println!(
                "Current Revit version: {}. Use change-revit-version if you want to select a different one.",
                revit_version
            );
        }
        Commands::ChangeRevitVersion => {
            let revit_version = prompt_for_revit_version();
            let state = state::get_state_or_default();
            state::save_state(&state::State {
                revit_version: revit_version.to_string(),
                ..state
            });
        }
        Commands::Locate => cmds::locate::execute(),
    }
}

pub fn ensure_revit_version_is_set() -> String {
    if let Some(state) = state::get_state() {
        state.revit_version
    } else {
        let revit_version = prompt_for_revit_version();
        let state = state::get_state_or_default();
        state::save_state(&state::State {
            revit_version: revit_version.to_string(),
            ..state
        });
        revit_version
    }
}
pub fn prompt_for_revit_version() -> String {
    println!("Enter the Revit version you want to use:");
    let mut revit_version = String::new();
    stdin().read_line(&mut revit_version).unwrap();
    let revit_version = revit_version.trim().to_string();
    if let Ok(revit_version) = revit_version.parse::<i32>() {
        if (2019..=2025).contains(&revit_version) {
            return revit_version.to_string();
        }
        println!("Invalid Revit version. You must specify a version between 2019 and 2025.");
        return prompt_for_revit_version();
    }
    println!(
        "Invalid Revit version. You must specify a version between 2019 and 2025. Include only the year."
    );
    prompt_for_revit_version()
}

// cargo install --path C:\Users\grieger.EMA\Desktop\Rust\rev
