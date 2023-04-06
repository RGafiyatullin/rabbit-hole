mod impl_point;
mod impl_scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scalar<F>(F);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<G>(G);
