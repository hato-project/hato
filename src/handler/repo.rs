use actix_web::{actix::Handler, error, Error};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use db::schema::repo::dsl::*;
use db::ConnDsl;
use model::{Repo, RepoID};

impl Handler<RepoID> for ConnDsl {
    type Result = Result<Option<Repo>, Error>;

    fn handle(&mut self, repo_id: RepoID, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let rp = repo
            .filter(&id.eq(&repo_id.repo_id))
            .load::<Repo>(conn)
            .map_err(error::ErrorInternalServerError)?
            .pop();
        Ok(rp)
    }
}
