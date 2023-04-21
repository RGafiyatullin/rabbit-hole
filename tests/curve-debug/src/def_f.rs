#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct F<const N: u64>(u64);

impl<const N: u64> F<N> {
    pub const fn from_u64(value: u64) -> Self {
        Self(value % N)
    }
    pub const fn from_i64(value: i64) -> Self {
        let value = if value.is_negative() { N - value.abs() as u64 } else { value as u64 };
        Self::from_u64(value)
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}
