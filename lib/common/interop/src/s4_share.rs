use serde::{Deserialize, Serialize};

use crate::types::{Point, Scalar};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S4Share {
    pub public_key: Point,
    pub shamir_x: Scalar,
    pub shamir_y: Scalar,
}
