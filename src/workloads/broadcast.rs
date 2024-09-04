use std::collections::HashSet;

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

    let broadcast_response = BroadcastData {
        src: node_id.to_string(),
        dest: generate_data.src.clone(),
        body: BroadcastBody {
            r#type: "broadcast_ok".to_string(),
            msg_id,
            in_reply_to: Some(generate_data.body.msg_id),
            message: None,
        },
    };

    print_json_to_stdout(broadcast_response)?;

    let old_sent = &env.received_messages[&message];
    let mut sent = env.received_messages[&message].clone();

    sent.insert(generate_data.src.clone());

    for neighbor in env
        .neighbors
        .iter()
        .filter(|n: &&String| !old_sent.contains(*n))
    {
        let broadcast = BroadcastData {
            src: node_id.to_string(),
            dest: neighbor.clone(),
            body: BroadcastBody {
                r#type: "broadcast".to_string(),
                msg_id,
                in_reply_to: None,
                message: Some(message),
            },
        };

        print_json_to_stdout(broadcast)?;

        sent.insert(neighbor.clone());
    }

    Ok(())
}
