use digest::Digest;
use elliptic_curve::point::AffineCoordinates;
use ff::PrimeField;
use group::{Curve, Group, GroupEncoding};
use rand::RngCore;

use crate::dkls_tss::THREE;

use super::{mta, Thrice, TWO};

pub fn presign_offer<F, G, const L: usize>(
    mut rng: impl RngCore,
    k_b: &mut F,
    d_b: &mut G,
    mta_a: &mut Thrice<impl AsMut<[F]>>,
    mta_d: &mut Thrice<impl AsMut<[F]>>,
    mta_pa: &mut Thrice<impl AsMut<[G]>>,
) where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    let g = G::generator();

    *k_b = F::random(&mut rng);
    *d_b = g * *k_b;

    for i in 0..THREE {
        let mta_a = mta_a[i].as_mut();
        let mta_d = mta_d[i].as_mut();
        let mta_pa = mta_pa[i].as_mut();
        hmrt_mta::sender_init::<F, G, L>(&mut rng, mta_d, mta_a, mta_pa);
    }
}

pub fn presign_reply<F, G, H, const L: usize>(
    r_seed: &G,
    t_0_b: &F,
    k_b: &F,
    d_b: &G,

    t_1_b: &mut F,
    t_2_b: &mut F,
    r: &mut G,

    mta_d: &Thrice<impl AsRef<[F]>>,
    mta_a: &Thrice<impl AsRef<[F]>>,
    mta_pb: &Thrice<impl AsRef<[G]>>,
    mta_s: &Thrice<impl AsRef<[F]>>,

    mta_e: &mut Thrice<impl AsMut<[[F; TWO]]>>,
) where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    *r = *r_seed + *d_b * utils::bytes_to_scalar::<F>(H::digest(r_seed.to_bytes()).as_ref());

    let k_b_inv = k_b.invert().unwrap();

    let mut additive_shares = [F::ZERO; THREE];

    for i in 0..THREE {
        let mta_d = mta_d[i].as_ref();
        let mta_a = mta_a[i].as_ref();
        let mta_pb = mta_pb[i].as_ref();
        let mta_e = mta_e[i].as_mut();
        let mta_s = mta_s[i].as_ref();

        let mult_share = match i {
            0 => k_b_inv,
            1 => k_b_inv,
            2 => *t_0_b * k_b_inv,
            _ => unreachable!(),
        };
        hmrt_mta::sender_ot_reply::<F, G, F, L>(
            &mult_share,
            mta_d,
            mta_a,
            mta_pb,
            mta_e,
            mta::encrypt::<F, G, H>,
        );
        additive_shares[i] = hmrt_mta::sender_additive_share::<F, L>(mta_s, mta_d);
    }

    *t_1_b = additive_shares[0];
    *t_2_b = additive_shares[1] + additive_shares[2];
}

pub fn sign<F, G, H>(
    public_key: &G,
    eta_phi: &F,
    eta_sig_a: &F,
    k_b: &F,
    t_1_b: &F,
    t_2_b: &F,
    r: &G,
    m: &F,
) -> F
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding + Curve,
    G::AffineRepr: AffineCoordinates<FieldRepr = F::Repr>,
    H: Digest,
{
    let g = G::generator();
    let r_x = F::from_repr(r.to_affine().x()).unwrap();

    let gamma_1 = *r * t_1_b;
    let h_1 = utils::bytes_to_scalar::<F>(H::digest(gamma_1.to_bytes()).as_ref());
    let phi = *eta_phi - h_1;

    let k_b_inv = k_b.invert().unwrap();
    let theta = *t_1_b - phi * k_b_inv;
    let gamma_2 = g * t_2_b - *public_key * theta;
    let h_2 = utils::bytes_to_scalar::<F>(H::digest(gamma_2.to_bytes()).as_ref());

    let sig_a = *eta_sig_a - h_2;
    let sig_b = *m * theta + r_x * t_2_b;

    sig_a + sig_b
}
