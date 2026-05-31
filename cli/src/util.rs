use anyhow::{Context, Result, bail};
use std::process::Command;

/// Run a command, streaming stdio to the user. Errors if the command fails.
pub fn bash(program: &str, args: &[&str]) -> Result<()> {
    println!("\n→ {} {}", program, args.join(" "));
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to spawn `{}`", program))?;

    if !status.success() {
        bail!("`{}` exited with status {}", program, status);
    }
    Ok(())
}

/// Run a shell command string via `/bin/bash -c`.
pub fn run_shell(cmd: &str) -> Result<()> {
    println!("\n→ sh: {}", cmd);
    let status = Command::new("/bin/bash")
        .arg("-c")
        .arg(cmd)
        .status()
        .context("failed to spawn bash")?;

    if !status.success() {
        bail!("shell command failed: {}", cmd);
    }
    Ok(())
}

/// Check whether a binary is available on $PATH.
pub fn has_binary(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
