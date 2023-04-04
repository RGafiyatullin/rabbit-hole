use ff::{Field, PrimeField};
use rand::RngCore;

pub trait SchemeInitFromSecret<F>: AsMut<[F]>
where
    F: PrimeField,
{
    fn init_from_secret(&mut self, secret: &F, mut rng: impl RngCore) {
        let coefficients = self.as_mut();
        coefficients[0] = *secret;

        coefficients[1..].iter_mut().for_each(|c| *c = F::random(&mut rng));
    }
}

pub trait SchemeIssueShare<F>: AsRef<[F]>
where
    F: Field,
{
    fn issue_share(&self, x: F) -> F {
        assert_ne!(x, F::ZERO);

        let (y, _) = self.as_ref().iter().copied().fold((F::ZERO, F::ONE), |(y, x_to_ith), c| {
            let term = c * x_to_ith;
            (y + term, x_to_ith * x)
        });

        y
    }
}

pub trait LagrangeCoefficientAt<F>: AsRef<[F]>
where
    F: Field,
{
    fn lagrange_coefficient_at(&self, i: usize, x: F) -> F {
        let xs = self.as_ref();

        let x_i = xs[i];
        xs[0..i]
            .iter()
            .chain(&xs[(i + 1)..])
            .copied()
            .map(|x_j| {
                let num = x - x_j;
                let den = x_i - x_j;
                let den_inv = den.invert().unwrap();
                num * den_inv
            })
            .product::<F>()
    }
}

impl<T, F> SchemeInitFromSecret<F> for T
where
    T: AsMut<[F]>,
    F: PrimeField,
{
}

impl<T, F> SchemeIssueShare<F> for T
where
    T: AsRef<[F]>,
    F: PrimeField,
{
}

impl<T, F> LagrangeCoefficientAt<F> for T
where
    T: AsRef<[F]>,
    F: Field,
{
}
