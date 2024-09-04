use crate::{
    types::topology::{TopologyBody, TopologyData},
    utils::print_json_to_stdout,
    Environment,
};

pub fn run_topology(node_id: &str, env: &Environment, line: &str) -> anyhow::Result<()> {
    let msg_id = env.msg_id;
    let generate_data: TopologyData = serde_json::from_str(&line)?;

    let topology_response = TopologyData {
        src: node_id.to_string(),
        dest: generate_data.src,
        body: TopologyBody {
            r#type: "topology_ok".to_string(),
            msg_id,
            in_reply_to: Some(generate_data.body.msg_id),
        },
    };

    print_json_to_stdout(topology_response)?;
    Ok(())
}
