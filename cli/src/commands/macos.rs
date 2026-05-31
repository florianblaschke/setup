use anyhow::Result;
use dialoguer::{Confirm, theme::ColorfulTheme};

use crate::util::bash as sh;

/// (description, args passed to `defaults`)
const TWEAKS: &[(&str, &[&str])] = &[
    (
        "Dock: enable autohide",
        &["write", "com.apple.dock", "autohide", "-bool", "true"],
    ),
    (
        "Dock: speed up autohide animation",
        &[
            "write",
            "com.apple.dock",
            "autohide-time-modifier",
            "-float",
            "0.1",
        ],
    ),
    (
        "Dock: remove autohide delay",
        &[
            "write",
            "com.apple.dock",
            "autohide-delay",
            "-float",
            "0",
        ],
    ),
    (
        "Dock: make hidden apps transparent",
        &[
            "write",
            "com.apple.dock",
            "showhidden",
            "-bool",
            "TRUE",
        ],
    ),
    (
        "Dock: hide recent apps",
        &[
            "write",
            "com.apple.dock",
            "show-recents",
            "-bool",
            "FALSE",
        ],
    ),
    (
        "Keyboard: faster key repeat",
        &["write", "-g", "KeyRepeat", "-int", "4"],
    ),
    (
        "Keyboard: shorter initial key repeat",
        &["write", "-g", "InitialKeyRepeat", "-int", "25"],
    ),
    (
        "Disable autocorrection",
        &[
            "write",
            "-g",
            "NSAutomaticSpellingCorrectionEnabled",
            "-bool",
            "false",
        ],
    ),
];

pub fn run() -> Result<()> {
    let theme = ColorfulTheme::default();

    if !Confirm::with_theme(&theme)
        .with_prompt("Apply the macOS \"nice-to-have\" defaults?")
        .default(true)
        .interact()?
    {
        return Ok(());
    }

    for (label, args) in TWEAKS {
        println!("• {label}");
        if let Err(e) = sh("defaults", args) {
            eprintln!("⚠ {label}: {e}");
        }
    }

    if Confirm::with_theme(&theme)
        .with_prompt("Restart Dock and Finder to apply changes?")
        .default(true)
        .interact()?
    {
        let _ = sh("killall", &["Dock"]);
        let _ = sh("killall", &["Finder"]);
    }

    Ok(())
}
