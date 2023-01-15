use colored::Colorize;
use index_client::IndexClient;
use miette::{Report, Result};
use rust_fuzzy_search::fuzzy_search_best_n;

use crate::config::get_config;

const MINIMUM_SCORE: f32 = 0.7;

pub struct Search;

impl Search {
    pub async fn execute(query: String) -> Result<()> {
        let config = get_config()?;

        let index_client = IndexClient::from_index_and_user_version(config.index, env!("CARGO_PKG_VERSION")).await?;
        let names = index_client.get_names().await?;
        let names_vec = names.iter().map(|name| name.as_str()).collect::<Vec<&str>>();
        let raw_fuzzy_results = fuzzy_search_best_n(&query, names_vec.as_slice(), 5);
        let matches: Vec<&(&str, f32)> = raw_fuzzy_results
            .iter()
            .filter(|(_, score)| score > &MINIMUM_SCORE)
            .collect();

        if matches.is_empty() {
            return Err(Report::msg(format!("{}", "No matches found.".red().bold())));
        }

        println!("{}", format!("{} matches found:", matches.len()).bold());

        for (result, _) in matches {
            println!("{}", format!(" - {result}").blue().bold())
        }

        Ok(())
    }
}
