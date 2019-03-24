use crate::model::user::UserData;
use bcrypt::{hash, verify};
use chrono::{Duration, Local};
use jwt::{decode, encode, Header, Validation};
use std::env;

use crate::errors::APIErrorKind;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub email: String,
    pub name:  String,
    pub iat:   i64,
    pub exp:   i64,
}
pub struct Extension(pub Claims);

pub fn encode_user_token(user: &UserData) -> String {
    let claims = Claims {
        email: user.email.clone(),
        name:  user.name.clone(),
        iat:   Local::now().timestamp(),
        exp:   (Local::now() + Duration::hours(24)).timestamp(),
    };
    let token = encode(&Header::default(), &claims, SECRET.as_ref()).unwrap();
    token
}

pub fn decode_user_token(token: &str) -> Result<Claims, APIErrorKind> {
    let result = decode::<Claims>(&token, SECRET.as_ref(), &Validation::default());
    match result {
        Ok(data) => Ok(data.claims),
        _ => Err(APIErrorKind::Unauthorized.into()),
    }
}

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
