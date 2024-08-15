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

pub fn get_echo_response(id: &str, msg_id: usize, line: &str) -> EchoData {
    let echo_data: EchoData = serde_json::from_str(&line).unwrap();
    let echo_response: EchoData = EchoData {
        src: id.to_string(),
        dest: echo_data.src,
        body: EchoBody {
            r#type: "echo_ok".to_string(),
            msg_id: msg_id,
            in_reply_to: Some(echo_data.body.msg_id),
            echo: echo_data.body.echo,
        },
    };

    echo_response
}
