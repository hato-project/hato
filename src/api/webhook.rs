use actix_web::{Json, Result};

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub before: String,
    pub after: String,
}


pub fn webhook(payload: Json<Payload>) -> Result<String> {
    Ok(format!("before {}", payload.before))
}
