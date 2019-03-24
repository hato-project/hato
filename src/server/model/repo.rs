use crate::db::schema::repo;
use chrono::prelude::*;

#[derive(Clone, Debug, Serialize, Insertable)]
#[table_name = "repo"]
pub struct NewRepo {
    pub name:      String,
    pub namespace: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable)]
pub struct Repo {
    pub name:       String,
    pub namespace:  String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
