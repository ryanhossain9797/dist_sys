use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitBody {
    pub r#type: String,
    pub msg_id: usize,
    pub node_id: String,
    pub node_ids: HashSet<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitData {
    pub src: String,
    pub dest: String,
    pub body: InitBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitResponseBody {
    pub r#type: String,
    pub in_reply_to: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitResponseData {
    pub src: String,
    pub dest: String,
    pub body: InitResponseBody,
}

pub fn get_init_response(line: &str) -> (InitResponseData, String, HashSet<String>) {
    let init_data: InitData = serde_json::from_str(&line).unwrap();
    let init_response = InitResponseData {
        src: init_data.body.node_id.clone(),
        dest: init_data.src,
        body: InitResponseBody {
            r#type: "init_ok".to_string(),
            in_reply_to: init_data.body.msg_id,
        },
    };

    (
        init_response,
        init_data.body.node_id,
        init_data.body.node_ids,
    )
}
