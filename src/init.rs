use std::collections::HashSet;

use crate::{
    types::init::{InitData, InitResponseBody, InitResponseData},
    utils::print_json_to_stdout,
};

pub async fn run_init(line: &str) -> anyhow::Result<(String, HashSet<String>)> {
    let init_data: InitData = serde_json::from_str(&line)?;

    let init_response = InitResponseData {
        src: init_data.body.node_id.clone(),
        dest: init_data.src,
        body: InitResponseBody {
            r#type: "init_ok".to_string(),
            in_reply_to: init_data.body.msg_id,
        },
    };

    print_json_to_stdout(&init_response).await?;

    Ok((init_data.body.node_id, init_data.body.node_ids))
}
