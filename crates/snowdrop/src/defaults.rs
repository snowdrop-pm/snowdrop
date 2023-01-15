use colored::Colorize;
use dialoguer::{console::style, theme::ColorfulTheme};

pub fn default_package_index() -> String {
    "https://raw.githubusercontent.com/snowdroppm/index/main".to_string()
}

pub fn theme() -> ColorfulTheme {
    ColorfulTheme {
        prompt_prefix: style(" ?".cyan().bold().to_string()),
        success_prefix: style(" ✔".green().to_string()),
        error_prefix: style(" ✘".red().to_string()),
        ..Default::default()
    }
}
