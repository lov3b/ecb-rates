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
        let os_str = env::consts::OS;
        if os_str == "windows" {
            Some(Os::Windows)
        } else if os_str == "macos" {
            Some(Os::Mac)
        } else if os_str == "linux" || os_str.contains("bsd") {
            Some(Os::Unix)
        } else {
            None
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
