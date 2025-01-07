use std::env;
use std::path::PathBuf;

use crate::APP_NAME;

pub enum Os {
    Windows,
    Mac,
    Unix,
}

impl Os {
    pub fn get_current() -> Option<Self> {
        match env::consts::OS {
            "windows" => Some(Self::Windows),
            "macos" => Some(Self::Mac),
            os_str if os_str == "linux" || os_str.contains("bsd") => Some(Self::Unix),
            _ => None,
        }
    }

    pub fn get_config_path(&self) -> Result<PathBuf, std::env::VarError> {
        let config_home = match self {
            Os::Windows => PathBuf::from(env::var("APPDATA")?),
            Os::Mac => {
                let mut pb = PathBuf::from(env::var("HOME")?);
                pb.push("Library");
                pb.push("Application Support");
                pb
            }
            Os::Unix => match env::var("XDG_CONFIG_HOME") {
                Ok(k) => PathBuf::from(k),
                Err(_) => {
                    let mut home = PathBuf::from(env::var("HOME")?);
                    home.push(".config");
                    home
                }
            },
        };

        Ok(config_home.join(APP_NAME))
    }
}
