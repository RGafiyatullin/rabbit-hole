use core::{fmt, str};

use group::GroupEncoding;
use serde::de::Error as DeError;
use serde::{Deserialize, Serialize};

use crate::curve_select::CurveSelect;
use crate::AnyError;

use super::Point;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

impl str::FromStr for Point {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((curve_select, hex_value)) = s.split_once(":") else  {
            return Err("should be <curve>:<hex>")
        };
        let curve_select = CurveSelect::from_str(curve_select).map_err(|_| "invalid curve")?;
        Ok(Self(curve_select, hex_value.to_string()))
    }
}

impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<Self>()
            .map_err(<D::Error as DeError>::custom)
    }
}

impl Point {
    pub fn restore<G: GroupEncoding>(&self, curve: CurveSelect) -> Result<G, AnyError> {
        if self.0 != curve {
            return Err("invalid curve".into())
        }

        let mut repr: G::Repr = Default::default();
        hex::decode_to_slice(self.1.as_str(), repr.as_mut())?;
        let out = G::from_bytes(&repr).unwrap();
        Ok(out)
    }
    pub fn new<G: GroupEncoding>(curve: CurveSelect, value: G) -> Self {
        let repr = value.to_bytes();
        let hex = hex::encode(repr.as_ref());
        Self(curve, hex)
    }
}

#[test]
fn test_serde_value() {
    let v1 = Point::new(CurveSelect::Secp256k1, k256::ProjectivePoint::GENERATOR * k256::Scalar::from(42u64));
    let s = serde_yaml::to_string(&v1).expect("ser");
    eprintln!("{}", s);
    let v2: Point = serde_yaml::from_str(&s).expect("de");
    assert_eq!(v1, v2);
}

#[test]
fn test_serde_key() {
    use std::collections::HashMap;

    let v1: HashMap<_, _> = [(
        Point::new(CurveSelect::Secp256k1, k256::ProjectivePoint::GENERATOR * k256::Scalar::from(42u64)),
        Point::new(CurveSelect::Secp256k1, k256::ProjectivePoint::GENERATOR * k256::Scalar::from(42u64))
    )].into_iter().collect();
    
    let s = serde_yaml::to_string(&v1).expect("ser");
    eprintln!("{}", s);
    let v2: HashMap<Point, Point> = serde_yaml::from_str(&s).expect("de");
    assert_eq!(v1, v2);
}
