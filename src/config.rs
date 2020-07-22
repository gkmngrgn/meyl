use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};
use tera;
use toml;

use crate::constants;

pub type Table = HashMap<String, toml::Value>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    base_url: Option<toml::Value>,
    context_data: Option<Table>,
}

impl Config {
    pub fn new(template_dir: PathBuf) -> std::io::Result<Self> {
        let config_path = template_dir.join(constants::FILE_CONFIG);
        let config_str = read_to_string(&config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    pub fn get_base_url(&self) -> String {
        match &self.base_url {
            Some(url) => url.as_str().unwrap().to_string(),
            None => "http://localhost".to_string(),
        }
    }

    pub fn get_context_data(&self) -> Table {
        match &self.context_data {
            Some(cd) => cd.clone(),
            None => Table::new(),
        }
    }
}

pub(crate) fn get_context(config: &Config) -> tera::Context {
    match tera::Context::from_serialize(&config.get_context_data()) {
        Ok(context) => context,
        Err(_) => tera::Context::new(),
    }
}
