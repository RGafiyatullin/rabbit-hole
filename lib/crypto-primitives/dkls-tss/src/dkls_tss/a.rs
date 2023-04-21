use digest::Digest;
use elliptic_curve::PrimeField;
use group::{Group, GroupEncoding};
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
    let g = G::generator();

    *phi = F::random(&mut rng);

    let k_a_seed = F::random(&mut rng);
    *r_seed = *d_b * k_a_seed;
    let h = utils::bytes_to_scalar::<F>(H::digest(r_seed.to_bytes()).as_ref());
    *k_a = h + k_a_seed;
    *r = *r_seed + g * h;

    assert_eq!(*r, *d_b * (h + k_a_seed));

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

pub fn sign<F, G, H>(k_a: &F, t_1_a: &F, t_2_a: &F, phi: &F, r: &G, m: &F) -> F
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    unimplemented!()
}
