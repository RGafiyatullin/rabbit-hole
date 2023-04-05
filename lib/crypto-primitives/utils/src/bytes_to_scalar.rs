use ff::PrimeField;

pub fn bytes_to_scalar<F>(bytes: &[u8]) -> F
where
    F: PrimeField,
{
    let mut repr: F::Repr = Default::default();

    let dst = repr.as_mut();
    let src = bytes;
    let buf_len = dst.len().min(src.len());

    let full_bytes = F::CAPACITY as usize / 8;
    let tail_bits = F::CAPACITY % 8;

    for i in 0..buf_len {
        use core::cmp::Ordering::*;
        match i.cmp(&full_bytes) {
            Greater => break,
            Less => dst[i] = src[i],
            Equal => {
                let mask = 0xFF >> (8 - tail_bits);
                dst[i] = src[i] & mask;
            },
        }
    }

    F::from_repr(repr).unwrap()
}
