use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EchoBody {
    pub r#type: String,
    pub msg_id: usize,
    pub in_reply_to: Option<usize>,
    pub echo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EchoData {
    pub src: String,
    pub dest: String,
    pub body: EchoBody,
}
