use actix_web::{Json, Result};

#[derive(Serialize, Deserialize)]
pub struct Payload {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub before: String,
    pub after: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Option<String>,
    pub compare: String,    
}


pub fn webhook(payload: Json<Payload>) -> Result<Json<Payload>> {
    Ok(payload)
}
