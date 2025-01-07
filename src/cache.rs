use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Context;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

use crate::models::ExchangeRateResult;
use crate::os::Os;

const FILE_NAME: &'static str = "cache.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,

    #[serde(rename = "camelCase")]
    pub exchange_rate_results: Vec<ExchangeRateResult>,
}

impl Cache {
    pub fn load() -> Option<Self> {
        let config_opt = Os::get_current()?.get_config_path();
        let mut config_path = match config_opt {
            Ok(k) => k,
            Err(e) => {
                eprintln!("Failed to locate config dir: {:?}", e);
                return None;
            }
        };
        if let Err(e) = fs::create_dir_all(&config_path) {
            eprintln!("Failed to create config dir: {:?}", e);
            return None;
        }
        config_path.push(FILE_NAME);
        if !config_path.try_exists().unwrap_or_default() {
            return None;
        }

        match Self::read_config(&config_path) {
            Ok(k) => Some(k),
            Err(e) => {
                eprintln!("Config path is invalid, or cannot be created: {:?}", e);
                None
            }
        }
    }

    pub fn new(exchange_rate_results: Vec<ExchangeRateResult>) -> Self {
        let date = Local::now().to_utc();
        Self {
            exchange_rate_results,
            date,
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let mut config_path = Os::get_current()
            .context("Failed to get config home")?
            .get_config_path()?;
        fs::create_dir_all(&config_path)?;

        config_path.push(FILE_NAME);

        let file = fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config_path)?;

        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;

        Ok(())
    }

    pub fn validate(&self) -> bool {
        let today = Local::now().naive_local().date();
        let saved = self.date.naive_local().date();
        saved == today
    }

    fn read_config(path: &Path) -> anyhow::Result<Self> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }
}
