#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HashFunctionSelect {
    Sha2_256,
    Sha3_256,
}

use std::{fmt, str};

use serde::de::Error as DeError;
use serde::{Deserialize, Serialize};

const SHA2_256: &str = "sha2-256";
const SHA3_256: &str = "sha3-256";

impl HashFunctionSelect {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sha2_256 => SHA2_256,
            Self::Sha3_256 => SHA3_256,
        }
    }
}

impl fmt::Display for HashFunctionSelect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for HashFunctionSelect {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            SHA3_256 => Self::Sha3_256,
            SHA2_256 => Self::Sha2_256,

            unknown =>
                return Err(format!(
                    "Unknown hash-function: {:?}. Known hash-functions: {}",
                    unknown,
                    [SHA2_256, SHA3_256].join(", ")
                )),
        };

        Ok(out)
    }
}

impl Serialize for HashFunctionSelect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HashFunctionSelect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(<D::Error as DeError>::custom)
    }
}
