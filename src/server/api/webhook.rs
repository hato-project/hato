use actix_web::{Json, Result};

use builder::common::create_dir;
use builder::git::clone_repo;

#[derive(Debug, Serialize, Deserialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub before: String,
    pub after: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Option<String>,
    pub compare: String,
    pub repository: Repository,
    pub pusher: Pusher,
    pub sender: Sender,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id:             u64,
    pub node_id:        String,
    pub name:           String,
    pub full_name:      String,
    pub private:        bool,
    pub html_url:       String,
    pub fork:           bool,
    pub url:            String,
    pub default_branch: String,
    pub master_branch:  String,
    pub created_at:     u64,
    pub updated_at:     String,
    pub pushed_at:      u64,
    pub git_url:        String,
    pub ssh_url:        String,
    pub clone_url:      String,
    pub svn_url:        String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pusher {
    pub name:  String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    #[serde(rename = "type")]
    pub type_: String,
}

pub fn webhook(payload: Json<PushEvent>) -> Result<Json<PushEvent>> {
    let work_dir = String::from("./tmp");
    create_dir(&work_dir);
    clone_repo(&payload.repository.clone_url, &work_dir);
    Ok(payload)
}
