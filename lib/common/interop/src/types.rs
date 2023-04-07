use serde::{Deserialize, Serialize};

mod impl_curve;
mod impl_hash_function;
mod impl_key_share;
mod impl_point;
mod impl_scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scalar<F>(F);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<G>(G);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "F: ff::PrimeField, G: group::GroupEncoding",
    deserialize = "F: ff::PrimeField, G: group::GroupEncoding"
))]
pub struct KeyShare<F, G> {
    pub public_key: Point<G>,
    pub shamir_x: Scalar<F>,
    pub shamir_y: Scalar<F>,
}

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
