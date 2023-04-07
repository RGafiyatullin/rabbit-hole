#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Curve {
    Secp256k1,
    Ed25519,
    Ristretto25519,
}

use std::{fmt, str};

const SECP256K1: &str = "secp256k1";
const ED25519: &str = "ed25519";
const RISTRETTO25519: &str = "ristretto25519";

impl Curve {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Secp256k1 => SECP256K1,
            Self::Ed25519 => ED25519,
            Self::Ristretto25519 => RISTRETTO25519,
        }
    }
}

impl fmt::Display for Curve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for Curve {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            SECP256K1 => Self::Secp256k1,
            ED25519 => Self::Ed25519,
            RISTRETTO25519 => Self::Ristretto25519,

            unknown =>
                return Err(format!(
                    "Unknown curve: {:?}. Known curves: {}",
                    unknown,
                    [SECP256K1, ED25519, RISTRETTO25519].join(", ")
                )),
        };

        Ok(out)
    }
}
