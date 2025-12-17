use std::{
    fmt,
    ops::Index,
    slice::Iter,
    str::{self, FromStr},
};

use serde::{de, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Currency {
    name: [u8; 3],
}

impl Currency {
    pub fn as_str(&self) -> &str {
        // SAFETY: We validate that bytes are ASCII in FromStr.
        unsafe { str::from_utf8_unchecked(&self.name) }
    }

    pub fn iter(&self) -> Iter<'_, u8> {
        self.name.iter()
    }
}

impl AsRef<str> for Currency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl FromStr for Currency {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            anyhow::bail!("Currency code must be exactly 3 chars");
        }
        if !s.is_ascii() {
            anyhow::bail!("Currency code must be ASCII");
        }

        let b = s.as_bytes();
        Ok(Self {
            name: [b[0], b[1], b[2]],
        })
    }
}

impl TryFrom<&str> for Currency {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Currency::from_str(value)
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Index<usize> for Currency {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.name[index]
    }
}

impl<'a> IntoIterator for &'a Currency {
    type Item = &'a u8;
    type IntoIter = Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
