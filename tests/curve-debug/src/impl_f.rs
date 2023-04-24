use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use ff::{Field, PrimeField};
use rand::RngCore;
use subtle::{ConditionallySelectable, ConstantTimeEq, CtOption};

pub(crate) const U32_MAX_PRIME: u64 = 4294967291;

use crate::F;

impl<const N: u64> From<u64> for F<N> {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}
impl<const N: u64> From<F<N>> for u64 {
    fn from(value: F<N>) -> Self {
        value.into_inner()
    }
}

impl<const N: u64> Neg for F<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        (N - self.into_inner()).into()
    }
}

impl<const N: u64> Add<Self> for F<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.into_inner() + rhs.into_inner()).into()
    }
}
impl<const N: u64> Add<&Self> for F<N> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        self + *rhs
    }
}

impl<const N: u64> Sub<Self> for F<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        #![allow(clippy::suspicious_arithmetic_impl)]

        self + rhs.neg()
    }
}
impl<const N: u64> Sub<&Self> for F<N> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        self - *rhs
    }
}

impl<const N: u64> Mul<Self> for F<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (self.into_inner() * rhs.into_inner()).into()
    }
}
impl<const N: u64> Mul<&Self> for F<N> {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        self * *rhs
    }
}

impl<const N: u64> AddAssign<Self> for F<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<const N: u64> AddAssign<&Self> for F<N> {
    fn add_assign(&mut self, rhs: &Self) {
        *self += *rhs;
    }
}

impl<const N: u64> SubAssign<Self> for F<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<const N: u64> SubAssign<&Self> for F<N> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self -= *rhs
    }
}

impl<const N: u64> MulAssign<Self> for F<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<const N: u64> MulAssign<&Self> for F<N> {
    fn mul_assign(&mut self, rhs: &Self) {
        *self *= *rhs;
    }
}

impl<const N: u64> Sum<Self> for F<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|l, r| l + r).unwrap_or(Self::ZERO)
    }
}
impl<'a, const N: u64> Sum<&'a Self> for F<N> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<const N: u64> Product<Self> for F<N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|l, r| l * r).unwrap_or(Self::ONE)
    }
}
impl<'a, const N: u64> Product<&'a Self> for F<N> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl<const N: u64> ConstantTimeEq for F<N> {
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        if self == other { 1u8 } else { 0u8 }.into()
    }
}

impl<const N: u64> ConditionallySelectable for F<N> {
    fn conditional_assign(&mut self, other: &Self, choice: subtle::Choice) {
        if choice.into() {
            *self = *other;
        }
    }

    fn conditional_select(a: &Self, b: &Self, choice: subtle::Choice) -> Self {
        if choice.into() {
            *a
        } else {
            *b
        }
    }
}

impl<const N: u64> Field for F<N> {
    const ZERO: Self = Self::from_u64(0);
    const ONE: Self = Self::from_u64(1);

    fn random(mut rng: impl RngCore) -> Self {
        rng.next_u64().into()
    }

    fn double(&self) -> Self {
        *self + self
    }

    fn square(&self) -> Self {
        *self * self
    }

    fn invert(&self) -> subtle::CtOption<Self> {
        fn ext_gcd(a: u64, b: u64) -> (u64, (i64, i64)) {
            // a*x + b*y = gcd

            if b == 0 {
                (a, (1, 0))
            } else {
                let (gcd, (x_1, y_1)) = ext_gcd(b, a % b);
                let x = y_1;
                let y = x_1 - (a as i64 / b as i64) * y_1;

                (gcd, (x, y))
            }
        }

        if self == &Self::ZERO {
            CtOption::new(Self::ZERO, 0u8.into())
        } else {
            let (_gcd, (i, _)) = ext_gcd(self.into_inner(), N);
            subtle::CtOption::new(Self::from_i64(i), 1u8.into())
        }
    }

    fn sqrt_ratio(_num: &Self, _div: &Self) -> (subtle::Choice, Self) {
        unimplemented!()
    }
}

impl PrimeField for F<U32_MAX_PRIME> {
    type Repr = [u8; 4];

    const CAPACITY: u32 = 32;
    const NUM_BITS: u32 = 32;
    const MODULUS: &'static str = "4294967290";
    const TWO_INV: Self = Self::from_u64(2147483646);
    const MULTIPLICATIVE_GENERATOR: Self = Self::from_u64(2);
    const S: u32 = 0;
    const ROOT_OF_UNITY: Self = Self::from_u64(1073741823);
    const ROOT_OF_UNITY_INV: Self = Self::from_u64(4);
    const DELTA: Self = Self::from_u64(4);

    fn from_repr(repr: Self::Repr) -> subtle::CtOption<Self> {
        subtle::CtOption::new(Self::from_u64(u32::from_ne_bytes(repr) as _), 1u8.into())
    }
    fn to_repr(&self) -> Self::Repr {
        (self.into_inner() as u32).to_ne_bytes()
    }

    fn is_odd(&self) -> subtle::Choice {
        if self.into_inner() % 2 == 1 { 1u8 } else { 0u8 }.into()
    }
}
