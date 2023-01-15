use colored::Colorize;
use dialoguer::Input;
use miette::{IntoDiagnostic, Result};
use regex::Regex;
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

use crate::{defaults::theme, dirs::get_project_dirs};

pub struct Auth;

impl Auth {
    pub async fn execute() -> Result<()> {
        let re = Regex::new(r"^(ghp_[a-zA-Z0-9]{36}|github_pat_[a-zA-Z0-9]{22}_[a-zA-Z0-9]{59}|v[0-9]\.[0-9a-f]{40})$")
            .unwrap();
        let pat_file_path = get_project_dirs()?.config_dir().join("pat.toml");

        println!(
            " Please enter a GitHub PAT {or} a GitHub Actions temporal token. You can make one at {url} {no_perms_needed}",
            or = "or".italic(),
            url = "https://github.com/settings/personal-access-tokens/new"
                .blue()
                .bold()
                .underline(),
            no_perms_needed = "(no permissions are required!)".magenta().bold()
        );
        let pat: String = Input::with_theme(&theme())
            .with_prompt("Your PAT")
            .validate_with(|input: &String| -> Result<(), &str> {
                if re.is_match(input) {
                    Ok(())
                } else {
                    Err("Invalid PAT token")
                }
            })
            .interact()
            .into_diagnostic()?;

        create_dir_all(pat_file_path.parent().unwrap())
            .await
            .into_diagnostic()?;
        let mut buffer = File::create(pat_file_path).await.into_diagnostic()?;
        buffer
            .write_all(format!("pat = \"{pat}\"").as_bytes())
            .await
            .into_diagnostic()?;

        Ok(())
    }
}
