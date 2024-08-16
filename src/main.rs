mod echo;
mod init;
mod types;
mod utils;

use std::collections::HashSet;

use tokio::io::{self, AsyncBufReadExt, BufReader, Lines, Stdin};

use echo::*;
use init::*;
use types::base::BaseData;
use utils::read_json_from_stdin;

pub async fn repl(
    mut reader: Lines<BufReader<Stdin>>,
    node_id: String,
    _node_ids: HashSet<String>,
) {
    let mut msg_id = 1;

    loop {
        let (data, line) = read_json_from_stdin::<BaseData>(&mut reader).await;
        match node_id == data.dest {
            true => match data.body.r#type.as_str() {
                "echo" => {
                    run_echo(node_id.as_str(), msg_id, &line).await;

                    msg_id = msg_id + 1;
                }
                _ => {}
            },
            false => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    let (init_data, init_line) = read_json_from_stdin::<BaseData>(&mut reader).await;
    match init_data.body.r#type.as_str() {
        "init" => {
            let (node_id, node_ids) = run_init(&init_line).await;

            repl(reader, node_id, node_ids).await;
        }
        _ => {}
    }
}
