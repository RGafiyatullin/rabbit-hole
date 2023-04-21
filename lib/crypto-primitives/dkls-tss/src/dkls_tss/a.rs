use digest::Digest;
use elliptic_curve::point::AffineCoordinates;
use ff::PrimeField;
use group::{Curve, Group, GroupEncoding};
use rand::RngCore;

use crate::dkls_tss::THREE;

use super::{mta, Thrice, TWO};

pub fn presign_choose<F, G, H, const L: usize>(
    mut rng: impl RngCore,

    t_0_a: &F,

    d_b: &G,
    phi: &mut F,
    k_a: &mut F,
    r: &mut G,
    r_seed: &mut G,

    mta_pa: &Thrice<impl AsRef<[G]>>,
    mta_pb: &mut Thrice<impl AsMut<[G]>>,
    mta_k: &mut Thrice<impl AsMut<[G]>>,
    mta_t: &mut Thrice<impl AsMut<[F]>>,
    mta_s: &mut Thrice<impl AsMut<[F]>>,
) where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    *phi = F::random(&mut rng);

    let k_a_seed = F::random(&mut rng);
    *r_seed = *d_b * k_a_seed;
    let h = utils::bytes_to_scalar::<F>(H::digest(r_seed.to_bytes()).as_ref());

    *k_a = h + k_a_seed;
    *r = *d_b * *k_a;

    assert_eq!(*r, *r_seed + *d_b * h);

    for i in 0..THREE {
        let mta_pa = mta_pa[i].as_ref();
        let mta_pb = mta_pb[i].as_mut();
        let mta_k = mta_k[i].as_mut();
        let mta_t = mta_t[i].as_mut();
        let mta_s = mta_s[i].as_mut();

        let k_a_inv = k_a.invert().unwrap();

        let mult_share = match i {
            0 => *phi + k_a_inv,
            1 => *t_0_a * k_a_inv,
            2 => k_a_inv,
            _ => unreachable!(),
        };

        hmrt_mta::receiver_ot_choose::<F, G, L>(
            &mut rng,
            &mult_share,
            mta_pa,
            mta_pb,
            mta_k,
            mta_t,
            mta_s,
        );
    }
}

pub fn presign_finalize<F, G, H, const L: usize>(
    t_1_a: &mut F,
    t_2_a: &mut F,

    mta_s: &Thrice<impl AsRef<[F]>>,
    mta_e: &Thrice<impl AsRef<[[F; TWO]]>>,
    mta_t: &Thrice<impl AsRef<[F]>>,
    mta_k: &Thrice<impl AsRef<[G]>>,
) where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    let mut additive_shares = [F::ZERO; THREE];

    for i in 0..THREE {
        let mta_s = mta_s[i].as_ref();
        let mta_e = mta_e[i].as_ref();
        let mta_t = mta_t[i].as_ref();
        let mta_k = mta_k[i].as_ref();

        additive_shares[i] = hmrt_mta::receiver_additive_share::<F, G, F, L>(
            mta_s,
            mta_e,
            mta_t,
            mta_k,
            mta::decrypt::<F, G, H>,
        );
    }

    *t_1_a = additive_shares[0];
    *t_2_a = additive_shares[1] + additive_shares[2];
}

pub fn sign<F, G, H>(
    public_key: &G,
    k_a: &F,
    t_1_a: &F,
    t_2_a: &F,
    phi: &F,
    r: &G,
    m: &F,
    eta_phi: &mut F,
    eta_sig_a: &mut F,
) where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding + Curve,
    G::AffineRepr: AffineCoordinates<FieldRepr = F::Repr>,
    H: Digest,
{
    let g = G::generator();

    let r_x = F::from_repr(r.to_affine().x()).unwrap();
    let sig_a = *m * t_1_a + r_x * t_2_a;

    let gamma_1 = g + g * k_a * phi - *r * t_1_a;
    let gamma_2 = *public_key * t_1_a - g * t_2_a;

    let h_1 = utils::bytes_to_scalar::<F>(H::digest(gamma_1.to_bytes()).as_ref());
    let h_2 = utils::bytes_to_scalar::<F>(H::digest(gamma_2.to_bytes()).as_ref());

    *eta_phi = h_1 + phi;
    *eta_sig_a = h_2 + sig_a;
}
