use cli_storage::AnyError;
use common_interop::hash_function_select::HashFunctionSelect;
use common_interop::transcript::{Input, KnownPoint, Transcript};
use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use specialize_call::specialize_call;

pub fn produce_challenge<F, G>(t: &Transcript, y: &G, r: &G) -> Result<F, AnyError>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    specialize_call!(produce_challenge_1, (t,y,r), t.hash_function,
        [
            (HashFunctionSelect::Sha3_256 => F, G, sha3::Sha3_256),
            (HashFunctionSelect::Sha2_256 => F, G, sha2::Sha256),
        ],
    )
    .ok_or(format!("unsupported hash-function: {}", t.hash_function))?
}

fn produce_challenge_1<F, G, H>(t: &Transcript, y: &G, r: &G) -> Result<F, AnyError>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    let mut hasher = H::new();

    for input in t.input.iter() {
        match input {
            Input::Hex(h) => hasher.update(hex::decode(h.as_str())?),
            Input::Text(t) => hasher.update(t),
            Input::Point(KnownPoint::Y) => hasher.update(y.to_bytes()),
            Input::Point(KnownPoint::R) => hasher.update(r.to_bytes()),
        }
    }

    Ok(utils::bytes_to_scalar(hasher.finalize().as_ref()))
}
