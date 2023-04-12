// #![no_std]

mod hmrt_mta;
// mod api_no_k;

pub use hmrt_mta::{
    receiver_additive_share, receiver_ot_choose, sender_additive_share, sender_init,
    sender_ot_reply,
};
