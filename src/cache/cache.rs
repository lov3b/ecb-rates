use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::cli::Resolution;
use crate::os::Os;

use super::CacheLine;

const FILE_NAME: &'static str = "cache.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    day: Option<CacheLine>,
    hist_90: Option<CacheLine>,
    hist_day: Option<CacheLine>,
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
            return Some(Self {
                day: None,
                hist_90: None,
                hist_day: None,
            });
        }

        match Self::read_config(&config_path) {
            Ok(k) => Some(k),
            Err(e) => {
                eprintln!("Config path is invalid, or cannot be created: {:?}", e);
                None
            }
        }
    }

    pub fn get_cache_line(&self, resolution: Resolution) -> Option<&CacheLine> {
        match resolution {
            Resolution::TODAY => self.day.as_ref(),
            Resolution::HistDays90 => self.hist_90.as_ref(),
            Resolution::HistDay => self.hist_day.as_ref(),
        }
    }

    pub fn set_cache_line(&mut self, resolution: Resolution, cache_line: CacheLine) {
        let cache_line_opt = Some(cache_line);
        match resolution {
            Resolution::TODAY => self.day = cache_line_opt,
            Resolution::HistDays90 => self.hist_90 = cache_line_opt,
            Resolution::HistDay => self.hist_day = cache_line_opt,
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

    fn read_config(path: &Path) -> anyhow::Result<Self> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }
}
