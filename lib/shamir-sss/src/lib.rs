#![no_std]

mod shamir_sss;

pub use crate::shamir_sss::{LagrangeCoefficientAt, SchemeInitFromSecret, SchemeIssueShare};
