use actix_web::actix::{Handler, Message};
use diesel::RunQueryDsl;

use crate::db::DbExecutor;
use crate::errors::APIErrorKind;
use crate::model::repo::{NewRepo, Repo};

#[derive(Debug, Deserialize)]
pub struct CreateRepo {
    pub namespace: String,
    pub name:      String,
}

impl Message for CreateRepo {
    type Result = Result<Repo, APIErrorKind>;
}

#[derive(Debug, Deserialize)]
pub struct UpdateRepo {
    pub namespace: String,
    pub name:      String,
}

impl Message for UpdateRepo {
    type Result = Result<Repo, APIErrorKind>;
}

#[derive(Debug, Deserialize)]
pub struct DeleteRepo {
    pub namespace: String,
    pub name:      String,
}

impl Message for DeleteRepo {
    type Result = Result<Repo, APIErrorKind>;
}

impl Handler<CreateRepo> for DbExecutor {
    type Result = Result<Repo, APIErrorKind>;

    fn handle(&mut self, msg: CreateRepo, _: &mut Self::Context) -> Self::Result {
        use db::schema::repo::dsl::repo;
        let conn = &self.0.get().map_err(|_| APIErrorKind::InternalError)?;
        let new_repo = NewRepo {
            namespace: msg.namespace,
            name:      msg.name,
        };
        let created_repo: Repo = diesel::insert_into(repo)
            .values(&new_repo)
            .get_result(conn)
            .map_err(|err| APIErrorKind::InternalError)?;
        Ok(created_repo)
    }
}
