use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadBody {
    pub r#type: String,
    pub msg_id: usize,
    pub in_reply_to: Option<usize>,
    #[serde(default)]
    pub messages: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadData {
    pub src: String,
    pub dest: String,
    pub body: ReadBody,
}
