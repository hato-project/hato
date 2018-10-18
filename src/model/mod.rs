use actix_web::{actix::Message, Error};
use chrono::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable)]
pub struct Repo {
    pub id:         String,
    pub name:       String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RepoID {
    pub repo_id: String,
}

impl Message for RepoID {
    type Result = Result<Option<Repo>, Error>;
}
