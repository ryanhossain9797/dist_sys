/*
{"src":"c1","dest":"n3","body":{"type":"init","msg_id":1,"node_id":"n3","node_ids":["n1", "n2","n3"]}}

{"src":"c1","dest":"n3","body":{"type":"echo","msg_id":1,"echo":"Please echo 35"}}

{"src":"c1","dest":"n3","body":{"type":"generate","msg_id":1}}
 */
mod echo;
mod generate;
mod init;
mod types;
mod utils;

use std::collections::HashSet;

use generate::run_generate;
use tokio::io::{self, AsyncBufReadExt, BufReader, Lines, Stdin};

use echo::*;
use init::*;
use types::base::BaseData;
use utils::read_json_from_stdin;

pub async fn repl(
    mut reader: Lines<BufReader<Stdin>>,
    node_id: String,
    _node_ids: HashSet<String>,
) -> anyhow::Result<()> {
    let mut msg_id = 1;

    loop {
        let (data, line) = read_json_from_stdin::<BaseData>(&mut reader).await?;
        eprintln!("{line}");
        match node_id == data.dest {
            true => {
                match data.body.r#type.as_str() {
                    "echo" => {
                        run_echo(node_id.as_str(), msg_id, &line).await?;
                    }
                    "generate" => {
                        run_generate(node_id.as_str(), msg_id, &line).await?;
                    }
                    _ => {}
                };
                msg_id = msg_id + 1;
            }
            false => Err(anyhow::anyhow!("Target Node Invalid"))?,
        }
    }
}

async fn start() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    let (init_data, init_line) = read_json_from_stdin::<BaseData>(&mut reader).await?;

    match init_data.body.r#type.as_str() {
        "init" => {
            let (node_id, node_ids) = run_init(&init_line).await?;

            repl(reader, node_id, node_ids).await
        }
        _ => Err(anyhow::anyhow!("Not Init")),
    }
}

#[tokio::main]
async fn main() {
    let failure = start().await;

    match failure {
        Ok(()) => {
            panic!("Unreacahble")
        }
        Err(err) => {
            eprint!("{err}")
        }
    }
}
