use anyhow::{Context, Result};
use config::{Config as ConfigR, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database: Database,
}

impl Config {
    pub fn new() -> Result<Self> {
        let mut s = ConfigR::default();
        s.merge(File::with_name(&format!("configs/default")))
            .context("Unable to load default.json")?;
        let env = env::var("ENV").unwrap_or("development".into());
        s.merge(File::with_name(&format!("configs/{}", env)).required(false))
            .context(format!("Unable to load config for env: {}", env))?;
        s.merge(Environment::new().separator("_".into()))?;
        s.try_into()
            .context("Unable to parse configs into settings struct")
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new().unwrap();
}
