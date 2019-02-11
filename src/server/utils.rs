use crate::model::user::UserData;
use bcrypt::{hash, verify};
use jwt::{decode, encode, Algorithm, Header, Validation};
use std::env;
use std::time::{Duration, Instant};

// static USER_TOKEN_EXPIRATION_TIME: Duration = Duration::new(60 * 60 * 24 * 7, 0);

lazy_static! {
    static ref SECRET: String = {
        match env::var("SECRET") {
            Ok(secret) => secret,
            _ => {
                println!("secret not set! use default");
                "dangerous".to_string()
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    name:  String,
}
pub fn encode_user_token(user: &UserData) -> String {
    let claims = Claims {
        email: user.email.clone(),
        name:  user.name.clone(),
    };
    let token = encode(&Header::default(), &claims, SECRET.as_ref()).unwrap();
    token
}

// pub fn decode_user_token(token: String) -> User {
//     unimplemented!()
// }

static DEFAULT_HASH_COST: u32 = 10;

lazy_static! {
    static ref HASH_COST: u32 = {
        match env::var("HASH_COST") {
            Ok(cost) => cost.parse().unwrap_or(DEFAULT_HASH_COST),
            _ => DEFAULT_HASH_COST,
        }
    };
}

pub fn hash_password(password: &str) -> String {
    hash(password, *HASH_COST).unwrap()
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap()
}
