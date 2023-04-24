use common_interop::curve_select::CurveSelect;
use common_interop::types::{Point, Scalar};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
    FullKey(FullKey),
    S4Share(S4Share),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullKey {
    pub curve: CurveSelect,
    pub value: Scalar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S4Share {
    pub curve: CurveSelect,
    pub threshold: usize,
    pub public_key: Point,
    pub x: Scalar,
    pub y: Scalar,
}
