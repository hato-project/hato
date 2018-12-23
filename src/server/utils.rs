use bcrypt::{hash, DEFAULT_COST};
use crate::errors::APIError;

use std::env;

pub fn hash_password(password: &str) -> Result<String, APIError> {
    let hash_cost: u32 = match env::var("HASH_COST") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    hash(password, hash_cost).map_err(|_| APIError::BadRequest)
}
