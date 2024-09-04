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
use std::io::{self, BufRead, StdinLock};

use generate::run_generate;

use echo::*;
use init::*;
use types::base::BaseData;
use utils::read_json_from_string;

pub fn repl(
    handle: StdinLock<'static>,
    node_id: String,
    _node_ids: HashSet<String>,
) -> anyhow::Result<()> {
    let mut msg_id = 1;

    for line in handle.lines() {
        let line = line?;
        let data = read_json_from_string::<BaseData>(&line)?;
        eprintln!("INPUT: {line}");
        match node_id == data.dest {
            true => {
                match data.body.r#type.as_str() {
                    "echo" => {
                        run_echo(node_id.as_str(), msg_id, &line)?;
                    }
                    "generate" => {
                        run_generate(node_id.as_str(), msg_id, &line)?;
                    }
                    _ => {}
                };
                msg_id = msg_id + 1;
            }
            false => Err(anyhow::anyhow!("Target Node Invalid"))?,
        }
    }

    Ok(())
}

async fn start() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock(); // Lock the stdin for reading

    let mut first_line = String::new();
    handle.read_line(&mut first_line)?;

    let init_data = read_json_from_string::<BaseData>(&first_line)?;

    match init_data.body.r#type.as_str() {
        "init" => {
            let (node_id, node_ids) = run_init(&first_line).await?;

            repl(handle, node_id, node_ids)
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
