use common_interop::types::{Point, Scalar};
use group::GroupEncoding;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(bound(serialize = "F: ::ff::PrimeField", deserialize = "F: ::ff::PrimeField"))]
pub struct Nonce<F> {
    pub d: Scalar<F>,
    pub e: Scalar<F>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(bound(serialize = "G: ::group::GroupEncoding", deserialize = "G: ::group::GroupEncoding"))]
pub struct Commitment<G> {
    pub cd: Point<G>,
    pub ce: Point<G>,
}

impl<G: GroupEncoding> Commitment<G> {
    pub fn to_storage_key(&self, s4_key_id: &str) -> String {
        format!("{}/{}:{}", s4_key_id, self.cd, self.ce)
    }
}
