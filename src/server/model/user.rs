use chrono::prelude::*;

use crate::db::schema::user;

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id:       i32,
    pub name:     String,
    pub nickname: String,
    pub email:    String,
    //     #[serde(skip_serializing)]
    //     pub token: Option<String>,
    //     #[serde(skip_serializing)]
    //     pub token_secret: Option<String>,
    //     #[serde(skip_serializing)]
    //     pub token_expiry: Option<String>,
    //     pub avatar: Option<String>,
    //     #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at:    NaiveDateTime,
    pub updated_at:    NaiveDateTime,
}

impl User {
    // pub fn set_pwd(mut self) -> Self {
    //     self
    // }

    // pub fn verify_pwd(mut self) -> bool {
    //     true
    // }
}
