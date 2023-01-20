use std::{collections::HashMap, env};

use globset::Glob;
use log::debug;
use maplit::hashmap;
use miette::{IntoDiagnostic, Result};
use octocrab::models::repos::Asset;

pub struct AssetPicker<'a> {
    format_data: HashMap<&'a str, &'a str>,
}

impl AssetPicker<'_> {
    pub fn new() -> Self {
        Self {
            format_data: hashmap! {
                "{{llvm_triple}}" => current_platform::CURRENT_PLATFORM,
                "{{basic_platform}}" => env::consts::OS,
                "{{basic_platform_osx}}" => match env::consts::OS {
                    "macos" => "osx",
                    _ => env::consts::OS
                }
            },
        }
    }

    pub fn choose_asset(&self, assets: Vec<Asset>, mut naming_scheme: String) -> Result<Asset> {
        for (key, value) in self.format_data.iter() {
            // TODO: Find a better way to do this!
            naming_scheme = naming_scheme.replace(key, value)
        }
        let glob = Glob::new(&naming_scheme).into_diagnostic()?.compile_matcher();
        for asset in assets {
            if glob.is_match(&asset.name) {
                debug!("Found a match: {}", asset.name);
                return Ok(asset);
            }
        }
        debug!("Didn't find a match, prompting user...");

        todo!()
    }
}

impl Default for AssetPicker<'_> {
    fn default() -> Self {
        Self::new()
    }
}
