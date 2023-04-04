use serde::{Deserialize, Serialize};

const CHALLENGE_SIZE: usize = 32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticateRequest(pub [u8; CHALLENGE_SIZE]);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticateResponse {}
