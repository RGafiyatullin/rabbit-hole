use serde::{Deserialize, Serialize};

use crate::hash_function::HashFunction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcript {
    pub hash_function: HashFunction,
    pub input: Vec<Input>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Input {
    Text(String),
    Hex(String),
    Point(KnownPoint),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnownPoint {
    Y,
    R,
}

#[test]
fn test_serde() {
    eprintln!(
        "{:#?}",
        serde_yaml::from_str::<Transcript>(
            "
    hash_function: sha3-256
    input:
        - !point Y
        - !point R
        - !text Hello There!
        - !hex 48656c6c6f20546865726521
    "
        )
        .expect("de")
    );
}
