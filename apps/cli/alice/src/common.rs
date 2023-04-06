use std::{fmt, str};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Curve {
    Secp256k1,
    Ed25519,
    Ristretto25519,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HashFunction {
    Sha3_256,
}

const SECP256K1: &str = "secp256k1";
const ED25519: &str = "ed25519";
const RISTRETTO25519: &str = "ristretto25519";

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

            unknown => return Err(format!("Unknown hash-function: {:?}", unknown)),
        };

        Ok(out)
    }
}

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

            unknown => return Err(format!("Unknown curve: {:?}", unknown)),
        };

        Ok(out)
    }
}