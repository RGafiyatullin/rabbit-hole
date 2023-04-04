use ff::Field;
use group::Group;

pub fn prove<F, G>(g: G, x: &F, k: &F, c: F) -> (F, G)
where
    G: Group<Scalar = F>,
    F: Field,
{
    let s = (c * x) + k;
    let r = g * k;
    (s, r)
}

pub fn verify<F, G>(g: G, y: G, c: F, s: F, r: G) -> bool
where
    G: Group<Scalar = F>,
    F: Field,
{
    g * s == r + y * c
}
