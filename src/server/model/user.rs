use crate::db::schema::user;
use chrono::prelude::*;

#[derive(Clone, Debug, Serialize, Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub name:          String,
    pub email:         String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct UserData {
    pub id: i32,
    pub name: String,
    pub nickname: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(skip_serializing)]
    pub token: Option<String>,
    #[serde(skip_serializing)]
    pub token_secret: Option<String>,
    #[serde(skip_serializing)]
    pub token_expiry: Option<String>,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
