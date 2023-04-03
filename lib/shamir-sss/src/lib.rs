#![no_std]

mod shamir_sss;

pub use shamir_sss::{LagrangeCoefficientAt, SchemeInitFromSecret, SchemeIssueShare};
