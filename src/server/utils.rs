use crate::errors::APIError;
use bcrypt::{hash, verify};

use std::env;

static DEFAULT_HASH_COST: u32 = 10;

pub fn hash_password(password: &str) -> String {
    let hash_cost: u32 = match env::var("HASH_COST") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_HASH_COST),
        _ => DEFAULT_HASH_COST,
    };
    hash(password, hash_cost).unwrap()
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap()
}
