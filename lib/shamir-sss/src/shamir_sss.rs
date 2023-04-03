use ff::{Field, PrimeField};
use rand::{CryptoRng, RngCore};

pub trait SchemeInitFromSecret<F>: AsMut<[F]>
where
    F: PrimeField,
{
    fn init_from_secret(&mut self, secret: F, rng: &mut (impl CryptoRng + RngCore)) {
        let coefficients = self.as_mut();
        coefficients[0] = secret;

        coefficients[1..].iter_mut().for_each(|c| {
            let mut repr: F::Repr = Default::default();
            rng.fill_bytes(repr.as_mut());
            *c = PrimeField::from_repr(repr).unwrap_or(Default::default());
        });
    }
}

pub trait SchemeIssueShare<F>: AsRef<[F]>
where
    F: Field,
{
    fn issue_share(&self, x: F) -> F {
        assert_ne!(x, F::ZERO);

        let mut y = F::ZERO;
        let mut x_to_nth = F::ONE;

        for c in self.as_ref() {
            y += x_to_nth * c;
            x_to_nth *= x;
        }

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
