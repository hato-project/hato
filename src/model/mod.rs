use actix_web::{actix::Message, Error};
use chrono::prelude::*;
use db::schema::repos;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable)]
pub struct Repo {
    pub id: String,
    pub name: String,
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

impl Repo {
    pub fn new() -> Repo {
        Repo {
            id: "".to_string(),
            name: "".to_string(),
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}
