use digest::Digest;
use elliptic_curve::PrimeField;
use group::GroupEncoding;

pub fn encrypt<F, G, H>(key: &G, n: &F) -> F
where
    F: PrimeField,
    G: GroupEncoding,
    H: Digest,
{
    let k: F = utils::bytes_to_scalar(H::digest(key.to_bytes()).as_ref());
    *n + k
}
pub fn decrypt<F, G, H>(key: &G, n: &F) -> F
where
    F: PrimeField,
    G: GroupEncoding,
    H: Digest,
{
    let k: F = utils::bytes_to_scalar(H::digest(key.to_bytes()).as_ref());
    *n - k
}
