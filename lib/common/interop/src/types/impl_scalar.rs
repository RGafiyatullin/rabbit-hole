use core::{fmt, str};

use ff::PrimeField;
use serde::de::Error as DeError;
use serde::{Deserialize, Serialize};

use crate::curve_select::CurveSelect;
use crate::AnyError;

use super::Scalar;

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

impl str::FromStr for Scalar {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((curve_select, hex_value)) = s.split_once(":") else  {
            return Err("should be <curve>:<hex>")
        };
        let curve_select = CurveSelect::from_str(curve_select).map_err(|_| "invalid curve")?;
        Ok(Self(curve_select, hex_value.to_string()))
    }
}

impl Serialize for Scalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Scalar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<Self>()
            .map_err(<D::Error as DeError>::custom)
    }
}

impl Scalar {
    pub fn restore<F: PrimeField>(&self, curve: CurveSelect) -> Result<F, AnyError> {
        if self.0 != curve {
            return Err("invalid curve".into())
        }

        let mut repr: F::Repr = Default::default();
        hex::decode_to_slice(self.1.as_str(), repr.as_mut())?;
        let out = F::from_repr(repr).unwrap();
        Ok(out)
    }
    pub fn new<F: PrimeField>(curve: CurveSelect, value: F) -> Self {
        let repr = value.to_repr();
        let hex = hex::encode(repr.as_ref());
        Self(curve, hex)
    }
}



#[test]
fn test_serde_value() {
    let v1 = Scalar::new(CurveSelect::Secp256k1, k256::Scalar::from(42u64));
    let s = serde_yaml::to_string(&v1).expect("ser");
    eprintln!("{}", s);
    let v2: Scalar = serde_yaml::from_str(&s).expect("de");
    assert_eq!(v1, v2);
}

#[test]
fn test_serde_key() {
    use std::collections::HashMap;
    
    let v1: HashMap<_, _> = [(
        Scalar::new(CurveSelect::Secp256k1, k256::Scalar::from(42u64)),
        Scalar::new(CurveSelect::Secp256k1, k256::Scalar::from(42u64))
    )].into_iter().collect();
    
    let s = serde_yaml::to_string(&v1).expect("ser");
    eprintln!("{}", s);
    let v2: HashMap<Scalar, Scalar> = serde_yaml::from_str(&s).expect("de");
    assert_eq!(v1, v2);
}
