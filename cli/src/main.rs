mod brew;
mod commands;
mod util;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mac-setup", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run the full setup wizard (init + git + macOS tweaks).
    All,
    /// Install Homebrew and pick which packages to install.
    Init,
    /// Configure git, SSH key and GitHub auth.
    Git,
    /// Apply the "nice-to-have" macOS defaults.
    Macos,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::All) {
        Command::All => {
            commands::init::run()?;
            commands::git::run()?;
            commands::macos::run()?;
        }
        Command::Init => commands::init::run()?,
        Command::Git => commands::git::run()?,
        Command::Macos => commands::macos::run()?,
    }

    println!("\n✨ all done!");
    Ok(())
}
