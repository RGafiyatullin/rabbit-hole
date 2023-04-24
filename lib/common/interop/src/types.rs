mod impl_point;
mod impl_scalar;

mod impl_kv;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scalar<F>(F);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<G>(G);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KV<K, V>(pub K, pub V);
