use ff::Field;
use group::Group;

pub trait CommitmentInitFromScheme<G, F>: AsMut<[G]>
where
    G: Group<Scalar = F>,
    F: Field,
{
    fn init_from_scheme(&mut self, scheme: &[F]) {
        let g = G::generator();

        let commitments = self.as_mut();

        assert!(scheme.len() <= commitments.len());

        commitments.iter_mut().for_each(|c| *c = G::identity());
        commitments.iter_mut().zip(scheme).for_each(|(c, s)| *c = g * s);
    }
}

pub trait CommitmentVerifyShare<G, F>: AsRef<[G]>
where
    G: Group<Scalar = F>,
    F: Field,
{
    fn verify_share(&self, x: &F, y: &F) -> bool {
        let g = G::generator();
        let cs = self.as_ref();

        let (c, _) = cs.iter().copied().fold((G::identity(), F::ONE), |(v, x_to_ith), c| {
            let term = c * x_to_ith;
            (v + term, x_to_ith * x)
        });

        let actual = c;
        let expected = g * y;

        // eprintln!("exp: {:0x?}", expected);
        // eprintln!("act: {:0x?}", actual);
        // eprintln!("---");

        expected == actual
    }
}

impl<T, G, F> CommitmentInitFromScheme<G, F> for T
where
    T: AsMut<[G]>,
    G: Group<Scalar = F>,
    F: Field,
{
}

impl<T, G, F> CommitmentVerifyShare<G, F> for T
where
    T: AsRef<[G]>,
    G: Group<Scalar = F>,
    F: Field,
{
}
