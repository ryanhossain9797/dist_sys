use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct InitBody {
    pub r#type: String,
    pub msg_id: usize,
    pub node_id: String,
    pub node_ids: HashSet<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InitData {
    pub src: String,
    pub dest: String,
    pub body: InitBody,
}

#[derive(Serialize, Debug, Clone)]
pub struct InitResponseBody {
    pub r#type: String,
    pub in_reply_to: usize,
}

#[derive(Serialize, Debug, Clone)]
pub struct InitResponseData {
    pub src: String,
    pub dest: String,
    pub body: InitResponseBody,
}
