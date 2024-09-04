use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopologyBody {
    pub r#type: String,
    pub msg_id: usize,
    pub in_reply_to: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopologyData {
    pub src: String,
    pub dest: String,
    pub body: TopologyBody,
}
