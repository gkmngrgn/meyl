use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tera;
use toml;

use crate::constants;

pub type Table = HashMap<String, toml::Value>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub context_data: Table,
}

fn parse_config(template_dir: PathBuf) -> std::io::Result<Config> {
    let config_path = template_dir.join(constants::FILE_CONFIG);
    let config_str = std::fs::read_to_string(&config_path).unwrap_or_default();
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

pub fn get_context_data(template_dir: PathBuf) -> tera::Context {
    match parse_config(template_dir) {
        Ok(config) => match tera::Context::from_serialize(&config.context_data) {
            Ok(context) => context,
            Err(_) => tera::Context::new(),
        },
        Err(_) => tera::Context::new(),
    }
}
