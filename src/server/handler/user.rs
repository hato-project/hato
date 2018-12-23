use actix_web::actix::{Handler, Message};
use diesel::RunQueryDsl;

use crate::db::DbExecutor;
use crate::errors::APIError;
use crate::model::user::User;
use crate::utils::hash_password;

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub name:     String,
    pub nickname: String,
    pub email:    String,
    pub password: String,
}

impl Message for RegisterUser {
    type Result = Result<User, APIError>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<User, APIError>;
    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::user::dsl::user;
        let conn = &self.0.get().map_err(|_| APIError::InternalError)?;
        let new_user = User {
            name:          msg.name,
            nickname:      msg.nickname,
            email:         msg.email,
            password_hash: hash_password(&msg.password).unwrap(),
            id:            None,
        };

        let inserted_user: User = diesel::insert_into(user)
            .values(&new_user)
            .get_result(conn)
            .map_err(|_| APIError::InternalError)?;
        Ok(inserted_user)
    }
}
