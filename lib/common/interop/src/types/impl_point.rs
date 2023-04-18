use core::{fmt, str};
use std::hash::Hash;

use group::GroupEncoding;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Point;

impl<G> AsRef<G> for Point<G> {
    fn as_ref(&self) -> &G {
        &self.0
    }
}

impl<G> AsMut<G> for Point<G> {
    fn as_mut(&mut self) -> &mut G {
        &mut self.0
    }
}

impl<G> From<G> for Point<G> {
    fn from(value: G) -> Self {
        Self(value)
    }
}

impl<G> Point<G> {
    pub fn into_inner(self) -> G {
        self.0
    }
}

impl<G> Hash for Point<G>
where
    G: GroupEncoding,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bytes().as_ref().hash(state)
    }
}

impl<G> fmt::Display for Point<G>
where
    G: GroupEncoding,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; SERDE_BUF_SIZE];

        let repr = self.0.to_bytes();
        let repr = repr.as_ref();
        let buf = &mut buf[0..(repr.len() * 2)];

        hex::encode_to_slice(repr, buf).map_err(|_| fmt::Error)?;
        let s = core::str::from_utf8(buf).map_err(|_| fmt::Error)?;

        write!(f, "{}", s)
    }
}

impl<G> str::FromStr for Point<G>
where
    G: GroupEncoding,
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut repr: G::Repr = Default::default();
        hex::decode_to_slice(s, repr.as_mut()).map_err(|_| ())?;
        let point = G::from_bytes(&repr);

        if point.is_some().into() {
            Ok(Self(point.unwrap()))
        } else {
            Err(())
        }
    }
}

const SERDE_BUF_SIZE: usize = 128;

impl<G: GroupEncoding> Serialize for Point<G> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        hex::encode(self.0.to_bytes().as_ref()).serialize(serializer)
    }
}

impl<'de, G: GroupEncoding> Deserialize<'de> for Point<G> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex: String = Deserialize::deserialize(deserializer)?;
        let mut repr: G::Repr = Default::default();
        hex::decode_to_slice(hex, repr.as_mut()).map_err(<D::Error as DeError>::custom)?;
        let point = G::from_bytes(&repr);

        if point.is_some().into() {
            Ok(Self(point.unwrap()))
        } else {
            Err(<D::Error as DeError>::custom("invalid repr"))
        }
    }
}

#[test]
fn point_serde() {
    let p_in: Point<_> = k256::ProjectivePoint::GENERATOR.into();
    let json = serde_json::to_string_pretty(&p_in).expect("ser");
    eprintln!("{}", json);
    let p_out: Point<k256::ProjectivePoint> = serde_json::from_str(&json).expect("de");

    assert_eq!(p_in, p_out);
}
