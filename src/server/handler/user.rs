use actix_web::actix::{Handler, Message};
use diesel::RunQueryDsl;

use crate::db::DbExecutor;
use crate::errors::APIError;
use crate::model::user::{NewUser, UserData};
use crate::utils::hash_password;

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub name:     String,
    pub email:    String,
    pub password: String,
}

impl Message for RegisterUser {
    type Result = Result<UserData, APIError>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<UserData, APIError>;
    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use db::schema::user::dsl::user;
        let conn = &self.0.get().map_err(|_| APIError::InternalError)?;
        let new_user = NewUser {
            name:          msg.name,
            email:         msg.email,
            password_hash: hash_password(&msg.password),
        };

        let inserted_user: UserData = diesel::insert_into(user)
            .values(&new_user)
            .get_result(conn)
            .map_err(|_| APIError::InternalError)?;
        Ok(inserted_user)
    }
}
