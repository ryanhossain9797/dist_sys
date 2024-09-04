use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BroadcastBody {
    pub r#type: String,
    pub msg_id: usize,
    pub in_reply_to: Option<usize>,
    pub message: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BroadcastData {
    pub src: String,
    pub dest: String,
    pub body: BroadcastBody,
}
