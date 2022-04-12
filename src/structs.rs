use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

// list of listnames and tasklists
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub lists: Vec<(String, Vec<(String, bool)>)>
}


