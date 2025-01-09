use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use super::CacheLine;
use crate::cli::View;
use crate::os::Os;

#[derive(Debug)]
pub struct Cache {
    cache_line: Option<CacheLine>,
    config_path: PathBuf,
}

impl Cache {
    pub fn load(view: View) -> Option<Self> {
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
        config_path.push(format!("{}.json", view.get_name()));
        if !config_path.try_exists().unwrap_or_default() {
            return Some(Self {
                cache_line: None,
                config_path,
            });
        }

        match Self::read_config(&config_path) {
            Ok(cache_line) => Some(Self {
                cache_line: Some(cache_line),
                config_path,
            }),
            Err(e) => {
                eprintln!("Config path is invalid, or cannot be created: {:?}", e);
                None
            }
        }
    }

    pub fn get_cache_line(&self) -> Option<&CacheLine> {
        self.cache_line.as_ref()
    }

    pub fn set_cache_line(&mut self, cache_line: CacheLine) {
        self.cache_line = Some(cache_line);
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let file = fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.config_path)?;

        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.cache_line)?;

        Ok(())
    }

    fn read_config(path: &Path) -> anyhow::Result<CacheLine> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }
}
