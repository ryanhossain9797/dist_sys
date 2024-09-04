use crate::{
    types::echo::{EchoBody, EchoData},
    utils::print_json_to_stdout,
    Environment,
};

pub fn run_echo(node_id: &str, env: &Environment, line: &str) -> anyhow::Result<()> {
    let msg_id = env.msg_id;
    let echo_data: EchoData = serde_json::from_str(line)?;

    let echo_response: EchoData = EchoData {
        src: node_id.to_string(),
        dest: echo_data.src,
        body: EchoBody {
            r#type: "echo_ok".to_string(),
            msg_id: msg_id,
            in_reply_to: Some(echo_data.body.msg_id),
            echo: echo_data.body.echo,
        },
    };

    print_json_to_stdout(echo_response)?;
    Ok(())
}
