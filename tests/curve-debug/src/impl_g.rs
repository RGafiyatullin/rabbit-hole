use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use elliptic_curve::point::AffineCoordinates;
use ff::PrimeField;
use group::{Curve, Group, GroupEncoding};

use crate::G;

impl<F: PrimeField> Neg for G<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(self.into_inner().neg())
    }
}

impl<F: PrimeField> Add<Self> for G<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.into_inner() + rhs.into_inner())
    }
}
impl<F: PrimeField> Add<&Self> for G<F> {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        self + *rhs
    }
}

impl<F: PrimeField> Sub<Self> for G<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.into_inner() - rhs.into_inner())
    }
}
impl<F: PrimeField> Sub<&Self> for G<F> {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        self - *rhs
    }
}

impl<F: PrimeField> AddAssign<Self> for G<F> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.into_inner() + rhs.into_inner());
    }
}
impl<F: PrimeField> AddAssign<&Self> for G<F> {
    fn add_assign(&mut self, rhs: &Self) {
        *self += *rhs;
    }
}

impl<F: PrimeField> SubAssign<Self> for G<F> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::new(self.into_inner() - rhs.into_inner());
    }
}
impl<F: PrimeField> SubAssign<&Self> for G<F> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self -= *rhs;
    }
}

impl<F: PrimeField> Sum<Self> for G<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|l, r| l + r).unwrap_or(Self::identity())
    }
}
impl<'a, F: PrimeField> Sum<&'a Self> for G<F> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|v| *v).sum()
    }
}

impl<F: PrimeField> Mul<F> for G<F> {
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        Self::new(self.into_inner() * rhs)
    }
}

impl<F: PrimeField> Mul<&F> for G<F> {
    type Output = Self;
    fn mul(self, rhs: &F) -> Self::Output {
        self * *rhs
    }
}

impl<F: PrimeField> MulAssign<F> for G<F> {
    fn mul_assign(&mut self, rhs: F) {
        *self = *self * rhs;
    }
}

impl<F: PrimeField> MulAssign<&F> for G<F> {
    fn mul_assign(&mut self, rhs: &F) {
        *self = *self * *rhs;
    }
}

impl<F: PrimeField> Group for G<F> {
    type Scalar = F;

    fn identity() -> Self {
        Self::new(F::ZERO)
    }
    fn generator() -> Self {
        Self::new(F::ONE)
    }
    fn double(&self) -> Self {
        Self::new(self.into_inner().double())
    }
    fn is_identity(&self) -> subtle::Choice {
        self.into_inner().ct_eq(&F::ZERO)
    }
    fn random(rng: impl rand::RngCore) -> Self {
        Self::new(F::random(rng))
    }
}

impl<F: PrimeField> GroupEncoding for G<F> {
    type Repr = F::Repr;

    fn from_bytes(bytes: &Self::Repr) -> subtle::CtOption<Self> {
        let f = F::from_repr(*bytes);
        f.map(Self::new)
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> subtle::CtOption<Self> {
        Self::from_bytes(bytes)
    }

    fn to_bytes(&self) -> Self::Repr {
        self.into_inner().to_repr()
    }
}

impl<F: PrimeField> Curve for G<F> {
    type AffineRepr = Self;

    fn to_affine(&self) -> Self::AffineRepr {
        *self
    }
}

impl<F: PrimeField> AffineCoordinates for G<F> {
    type FieldRepr = F::Repr;

    fn x(&self) -> Self::FieldRepr {
        self.into_inner().to_repr()
    }

    fn y_is_odd(&self) -> subtle::Choice {
        unimplemented!()
    }
}
