use core::{fmt, str};
use std::hash::Hash;

use ff::PrimeField;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Scalar;

impl<F> AsRef<F> for Scalar<F> {
    fn as_ref(&self) -> &F {
        &self.0
    }
}

impl<F> AsMut<F> for Scalar<F> {
    fn as_mut(&mut self) -> &mut F {
        &mut self.0
    }
}

impl<F> From<F> for Scalar<F> {
    fn from(value: F) -> Self {
        Self(value)
    }
}

impl<F> Scalar<F> {
    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<F> Hash for Scalar<F>
where
    F: PrimeField,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_repr().as_ref().hash(state)
    }
}

const SERDE_BUF_SIZE: usize = 128;

impl<F> fmt::Display for Scalar<F>
where
    F: PrimeField,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; SERDE_BUF_SIZE];
        let len = F::NUM_BITS as usize / 8 * 2;

        let buf = &mut buf[0..len];

        hex::encode_to_slice(self.0.to_repr().as_ref(), buf).map_err(|_| fmt::Error)?;
        let s = core::str::from_utf8(&buf[0..len]).map_err(|_| fmt::Error)?;

        write!(f, "{}", s)
    }
}

impl<F> str::FromStr for Scalar<F>
where
    F: PrimeField,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut repr: F::Repr = Default::default();
        hex::decode_to_slice(s, &mut repr.as_mut()[0..(s.len() / 2)]).map_err(|_| ())?;
        let scalar = F::from_repr(repr);

        if scalar.is_some().into() {
            Ok(Self(scalar.unwrap()))
        } else {
            Err(())
        }
    }
}

impl<F: PrimeField> Serialize for Scalar<F> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        hex::encode(self.0.to_repr().as_ref()).serialize(serializer)
    }
}

impl<'de, F: PrimeField> Deserialize<'de> for Scalar<F> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex: String = Deserialize::deserialize(deserializer)?;

        let mut repr: F::Repr = Default::default();
        hex::decode_to_slice(hex.as_str(), &mut repr.as_mut()[0..hex.len() / 2])
            .map_err(<D::Error as DeError>::custom)?;
        let scalar =
            F::from_repr_vartime(repr).ok_or(<D::Error as DeError>::custom("invalid repr"))?;

        Ok(Self(scalar))
    }
}

#[test]
fn scalar_serde_1() {
    use ff::Field;

    let s_in: Scalar<_> = k256::Scalar::random(&mut rand::rngs::OsRng).into();
    let json = serde_json::to_string_pretty(&s_in).expect("ser");
    let s_out: Scalar<k256::Scalar> = serde_json::from_str(&json).expect("de");

    assert_eq!(s_in, s_out);
}

#[test]
fn scalar_serde_2() {
    let _s: Scalar<k256::Scalar> = "01".parse().expect("parse");
    let _s: Scalar<k256::Scalar> = serde_json::from_str("\"01\"").expect("de");
}

#[test]
fn scalar_serde_3() {
    use std::collections::HashMap;

    let map: HashMap<Scalar<k256::Scalar>, u64> =
        [(Scalar(2u64.into()), 2), (Scalar(4u64.into()), 4)].into_iter().collect();
    let json = serde_json::to_string_pretty(&map).expect("ser");
    eprintln!("{}", json);
}
