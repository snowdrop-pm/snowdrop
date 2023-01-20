use colored::Colorize;
use index_client::IndexClient;
use miette::{Report, Result};
use rust_fuzzy_search::fuzzy_search_best_n;

use crate::config::get_config;

pub struct Search;

impl Search {
    pub async fn execute(query: String, minimum_score: &f32) -> Result<()> {
        let config = get_config()?;
        let pat = config.get_pat()?;

        let index_client = IndexClient::new(&config.index, env!("CARGO_PKG_VERSION"), pat.clone()).await?;
        let names = index_client.get_names().await?;
        let names_vec = names.iter().map(|name| name.as_str()).collect::<Vec<&str>>();
        let raw_fuzzy_results = fuzzy_search_best_n(&query, names_vec.as_slice(), 5);
        let matches: Vec<&(&str, f32)> = raw_fuzzy_results
            .iter()
            .filter(|(_, score)| score >= minimum_score)
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
