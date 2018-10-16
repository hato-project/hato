use actix_web::{actix::Handler, error, Error};
use db::ConnDsl;
use diesel::{self, sql_query, ExpressionMethods, QueryDsl, RunQueryDsl};
use model::{Repo, RepoID};

impl Handler<RepoID> for ConnDsl {
    type Result = Result<Option<Repo>, Error>;

    fn handle(&mut self, repo_id: RepoID, _: &mut Self::Context) -> Self::Result {
        use db::schema::repos::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let the_repo = repos
            .filter(&id.eq(&repo_id.repo_id))
            .load::<Repo>(conn)
            .map_err(error::ErrorInternalServerError)?
            .pop();
        match the_repo {
            Some(the_repo) => {
                let repo = Repo {
                    id: the_repo.id,
                    name: the_repo.name,
                    created_at: the_repo.created_at.clone(),
                    updated_at: the_repo.updated_at.clone(),
                };
                Ok(Some(repo))
            }
            None => Ok(None),
        }
    }
}
