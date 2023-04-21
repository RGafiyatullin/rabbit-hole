#![no_std]

mod def_f;
mod def_g;
mod impl_f;
mod impl_g;

pub use def_f::F;
pub use def_g::G;

pub type FU32 = def_f::F<{ impl_f::U32_MAX_PRIME }>;
pub type GU32 = G<FU32>;
