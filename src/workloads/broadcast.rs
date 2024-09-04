use crate::{
    types::broadcast::{BroadcastBody, BroadcastData},
    utils::print_json_to_stdout,
    Environment,
};

pub fn run_broadcast(node_id: &str, env: &mut Environment, line: &str) -> anyhow::Result<()> {
    let msg_id = env.msg_id;
    let generate_data: BroadcastData = serde_json::from_str(&line)?;

    let message = generate_data
        .body
        .message
        .ok_or_else(|| anyhow::anyhow!("No Message"))?;

    env.received_messages.push(message);

    let broadcast_response = BroadcastData {
        src: node_id.to_string(),
        dest: generate_data.src,
        body: BroadcastBody {
            r#type: "broadcast_ok".to_string(),
            msg_id,
            in_reply_to: Some(generate_data.body.msg_id),
            message: None,
        },
    };

    print_json_to_stdout(broadcast_response)?;

    for neighbor in env.neighbors.iter() {
        let broadcast_response = BroadcastData {
            src: node_id.to_string(),
            dest: neighbor.clone(),
            body: BroadcastBody {
                r#type: "broadcast".to_string(),
                msg_id,
                in_reply_to: None,
                message: Some(message),
            },
        };
    }

    Ok(())
}
