use anyhow::Result;
use dialoguer::{Confirm, Input, theme::ColorfulTheme};

use crate::util::{bash as sh, run_shell};

pub fn run() -> Result<()> {
    let theme = ColorfulTheme::default();

    let full_name: String = Input::with_theme(&theme)
        .with_prompt("Enter your full name (e.g. John Doe)")
        .interact_text()?;

    let email: String = Input::with_theme(&theme)
        .with_prompt("Enter the e-mail you use for GitHub")
        .interact_text()?;

    sh("git", &["config", "--global", "user.name", &full_name])?;
    sh("git", &["config", "--global", "user.email", &email])?;
    sh("git", &["config", "--global", "pull.ff", "only"])?;
    sh("git", &["config", "--global", "init.defaultBranch", "main"])?;

    // SSH key
    if Confirm::with_theme(&theme)
        .with_prompt("Generate a new ed25519 SSH key?")
        .default(true)
        .interact()?
    {
        let home = std::env::var("HOME")?;
        run_shell("mkdir -p ~/.ssh")?;
        sh("ssh-keygen", &["-t", "ed25519", "-C", &email, "-f", &format!("{}/.ssh/id_ed25519", home), "-N", ""])?;
        run_shell(r#"eval "$(ssh-agent -s)""#)?;

        let ssh_config = format!("{home}/.ssh/config");
        run_shell(&format!(
            r#"printf '%s\n' 'Host *' '    AddKeysToAgent yes' '    UseKeychain yes' '    IdentityFile ~/.ssh/id_ed25519' > {ssh_config}"#
        ))?;
        sh("ssh-add", &["--apple-use-keychain", &format!("{}/.ssh/id_ed25519", home)])?;
        run_shell("ssh-keyscan -t rsa github.com >> ~/.ssh/known_hosts")?;
    }

    // gh auth
    if Confirm::with_theme(&theme)
        .with_prompt("Authenticate with GitHub via `gh auth login`?")
        .default(true)
        .interact()?
    {
        sh("gh", &["auth", "login"])?;
    }

    Ok(())
}
