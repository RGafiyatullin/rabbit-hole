#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "F: ff::PrimeField, G: group::GroupEncoding",
    deserialize = "F: ff::PrimeField, G: group::GroupEncoding"
))]
pub struct S4Share<F, G> {
    pub public_key: Point<G>,
    pub shamir_x: Scalar<F>,
    pub shamir_y: Scalar<F>,
}

use core::{fmt, str};

use ff::PrimeField;
use group::GroupEncoding;
use serde::{Deserialize, Serialize};

use crate::types::{Point, Scalar};

impl<F, G> fmt::Display for S4Share<F, G>
where
    F: PrimeField,
    G: GroupEncoding,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.public_key, self.shamir_x, self.shamir_y)
    }
}

impl<F, G> str::FromStr for S4Share<F, G>
where
    F: PrimeField,
    G: GroupEncoding,
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (public_key, s) = s.split_once(':').ok_or(())?;
        let (shamir_x, shamir_y) = s.split_once(':').ok_or(())?;

        Ok(Self {
            public_key: public_key.parse()?,
            shamir_x: shamir_x.parse()?,
            shamir_y: shamir_y.parse()?,
        })
    }
}
