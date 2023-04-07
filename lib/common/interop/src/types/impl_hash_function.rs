use std::{fmt, str};

use super::HashFunction;

const SHA3_256: &str = "sha3-256";

impl HashFunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sha3_256 => SHA3_256,
        }
    }
}

impl fmt::Display for HashFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for HashFunction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            SHA3_256 => Self::Sha3_256,

            unknown =>
                return Err(format!(
                    "Unknown hash-function: {:?}. Known hash-functions: {}",
                    unknown,
                    [SHA3_256].join(", ")
                )),
        };

        Ok(out)
    }
}
