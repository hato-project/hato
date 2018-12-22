pub struct User {
    pub id:         u64,
    pub name:       String,
    pub nickname: String,
    pub email: String,
    pub token: String,
    pub token_secret String,
    pub token_expiry String,
    pub avatar String,
    pub password_hash String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
