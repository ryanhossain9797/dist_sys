use crate::{
    types::read::{ReadBody, ReadData},
    utils::print_json_to_stdout,
    Environment,
};

pub fn run_read(node_id: &str, env: &Environment, line: &str) -> anyhow::Result<()> {
    let msg_id = env.msg_id;
    let generate_data: ReadData = serde_json::from_str(&line)?;

    let read_response = ReadData {
        src: node_id.to_string(),
        dest: generate_data.src,
        body: ReadBody {
            r#type: "read_ok".to_string(),
            msg_id,
            in_reply_to: Some(generate_data.body.msg_id),
            messages: env.received_messages.clone(),
        },
    };

    print_json_to_stdout(read_response)?;
    Ok(())
}
