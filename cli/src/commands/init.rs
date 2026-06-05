use anyhow::Result;
use dialoguer::{Confirm, MultiSelect, theme::ColorfulTheme};

use crate::brew::PACKAGES;
use crate::util::{bash, has_binary, run_shell};

const BREW_INSTALL: &str = r#"/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#;
const ZSHRC_URL: &str =
    "https://raw.githubusercontent.com/florianblaschke/setup/main/configs/.zshrc";
const P10K_URL: &str =
    "https://raw.githubusercontent.com/florianblaschke/setup/main/configs/.p10k.zsh";

pub fn run() -> Result<()> {
    let theme = ColorfulTheme::default();

    // 1. Homebrew
    if !has_binary("brew") {
        if Confirm::with_theme(&theme)
            .with_prompt("Homebrew is not installed. Install it now?")
            .default(true)
            .interact()?
        {
            run_shell(BREW_INSTALL)?;
            run_shell(r#"eval "$(/opt/homebrew/bin/brew shellenv)""#)?;
        }
    } else {
        println!("✓ Homebrew already installed, skipping.");
    }

    // 2. Pick packages
    let defaults: Vec<bool> = vec![true; PACKAGES.len()];
    let selection = MultiSelect::with_theme(&theme)
        .with_prompt("Select brew packages to install (space to toggle, enter to confirm)")
        .items(PACKAGES)
        .defaults(&defaults)
        .interact()?;

    for i in selection {
        let pkg = PACKAGES[i];
        if let Err(e) = bash("brew", &["install", pkg]) {
            eprintln!("⚠ failed to install {pkg}: {e}");
        }
    }

    // 3. Copy config files
    if Confirm::with_theme(&theme)
        .with_prompt("Download .zshrc and .p10k.zsh into your home directory?")
        .default(true)
        .interact()?
    {
        let home = std::env::var("HOME")?;
        run_shell(&format!("curl -sL {ZSHRC_URL} > {home}/.zshrc"))?;
        run_shell(&format!("curl -sL {P10K_URL} > {home}/.p10k.zsh"))?;
    }

    Ok(())
}
