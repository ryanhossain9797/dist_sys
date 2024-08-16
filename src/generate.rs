use chrono::Utc;

use crate::{
    types::generate::{GenerateBody, GenerateData},
    utils::print_json_to_stdout,
};

pub async fn run_generate(node_id: &str, msg_id: usize, line: &str) {
    let generate_data: GenerateData = serde_json::from_str(&line).unwrap();

    let now = Utc::now();
    let utc_ticks = now.timestamp_micros();

    let generate_response: GenerateData = GenerateData {
        src: node_id.to_string(),
        dest: generate_data.src,
        body: GenerateBody {
            r#type: "generate_ok".to_string(),
            msg_id: msg_id,
            in_reply_to: Some(generate_data.body.msg_id),
            id: Some(format!("{node_id}{utc_ticks}{msg_id}")),
        },
    };

    print_json_to_stdout(generate_response).await;
}
