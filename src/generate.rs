use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    types::generate::{GenerateBody, GenerateData},
    utils::print_json_to_stdout,
};

pub fn run_generate(node_id: &str, msg_id: usize, line: &str) -> anyhow::Result<()> {
    let generate_data: GenerateData = serde_json::from_str(&line)?;

    let now = SystemTime::now();

    // Calculate the duration since the Unix epoch
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let utc_ticks = duration_since_epoch.as_micros();

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

    print_json_to_stdout(generate_response)?;
    Ok(())
}
