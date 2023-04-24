use serde::{Deserialize, Serialize};

use crate::curve_select::CurveSelect;

mod impl_point;
mod impl_scalar;

mod impl_kv;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Scalar(CurveSelect, String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point(CurveSelect, String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KV<K, V>(pub K, pub V);
