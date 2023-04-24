use common_interop::types::{Point, Scalar};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "F: ::ff::PrimeField, G: ::group::GroupEncoding",
    deserialize = "F: ::ff::PrimeField, G: ::group::GroupEncoding"
))]
pub struct Session<F, G> {
    pub shamir_x: Scalar<F>,
    pub shamir_y: Scalar<F>,

    pub commitment: Vec<Point<G>>,
}
