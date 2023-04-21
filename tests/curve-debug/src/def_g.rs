#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct G<F>(F);

impl<F> G<F> {
    pub const fn new(value: F) -> Self {
        Self(value)
    }
    pub(crate) fn into_inner(self) -> F {
        self.0
    }
}
