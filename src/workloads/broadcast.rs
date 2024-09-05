use tokio::{io::Stdout, sync::mpsc::UnboundedReceiver};

use crate::{
    types::{base::BaseData, broadcast::BroadcastBody},
    utils::print_json_to_stdout,
    Environment,
};

pub async fn outbound_broadcast_queue(
    mut receiver: UnboundedReceiver<BaseData<BroadcastBody>>,
) -> anyhow::Result<!> {
    let mut writer = tokio::io::stdout();
    while let Some(broadcast) = receiver.recv().await {
        print_json_to_stdout(&mut writer, broadcast).await?;
    }

    Err(anyhow::anyhow!("Unreachable"))
}

pub async fn run_broadcast(
    writer: &mut Stdout,
    node_id: &str,
    env: &mut Environment,
    line: &str,
) -> anyhow::Result<()> {
    let msg_id = env.msg_id;
    let generate_data: BaseData<BroadcastBody> = serde_json::from_str(&line)?;

    let message = generate_data
        .body
        .message
        .ok_or_else(|| anyhow::anyhow!("No Message"))?;

    let broadcast_response = BaseData {
        src: node_id.to_string(),
        dest: generate_data.src.clone(),
        body: BroadcastBody {
            r#type: "broadcast_ok".to_string(),
            msg_id,
            in_reply_to: Some(generate_data.body.msg_id),
            message: None,
        },
    };

    print_json_to_stdout(writer, broadcast_response).await?;

    let old_sent = env
        .received_messages
        .get(&message)
        .map(|s| s.clone())
        .unwrap_or_default();

    let mut sent = old_sent.clone();

    sent.insert(generate_data.src.clone());

    for neighbor in env
        .neighbors
        .iter()
        .filter(|n: &&String| !old_sent.contains(*n))
    {
        let broadcast = BaseData {
            src: node_id.to_string(),
            dest: neighbor.clone(),
            body: BroadcastBody {
                r#type: "broadcast".to_string(),
                msg_id,
                in_reply_to: None,
                message: Some(message),
            },
        };

        env.broadcast_sender.send(broadcast)?;

        sent.insert(neighbor.clone());

        env.msg_id += 1;
    }

    env.received_messages.insert(message, sent);

    Ok(())
}
