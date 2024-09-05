/*
{"src":"c1","dest":"n3","body":{"type":"init","msg_id":1,"node_id":"n3","node_ids":["n1", "n2","n3"]}}

{"src":"c1","dest":"n3","body":{"type":"echo","msg_id":1,"echo":"Please echo 35"}}

{"src":"c1","dest":"n3","body":{"type":"generate","msg_id":1}}
 */
#![feature(never_type)]
mod init;
mod types;
mod utils;
mod workloads;

use std::collections::{HashMap, HashSet};

use init::*;
use tokio::io::{AsyncBufReadExt, BufReader, Lines, Stdin, Stdout};
use tokio::sync::mpsc::{Sender, UnboundedSender};
use types::base::BaseData;
use types::broadcast::BroadcastData;
use utils::read_json_from_string;

use workloads::broadcast::{outbound_broadcast_queue, run_broadcast};
use workloads::echo::run_echo;
use workloads::generate::run_generate;
use workloads::read::run_read;
use workloads::topology::run_topology;

struct Environment {
    msg_id: usize,
    received_messages: HashMap<usize, HashSet<String>>,
    neighbors: HashSet<String>,
    broadcast_sender: UnboundedSender<BroadcastData>,
}

pub async fn repl(
    mut lines: Lines<BufReader<Stdin>>,
    mut writer: Stdout,
    node_id: String,
    _node_ids: HashSet<String>,
) -> anyhow::Result<()> {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<BroadcastData>();

    tokio::spawn(outbound_broadcast_queue(receiver));
    let mut env = Environment {
        msg_id: 1,
        received_messages: HashMap::new(),
        neighbors: HashSet::new(),
        broadcast_sender: sender,
    };

    while let Some(line) = lines.next_line().await? {
        let data = read_json_from_string::<BaseData>(&line)?;
        eprintln!("INPUT: {line}");
        match node_id == data.dest {
            true => {
                match data.body.r#type.as_str() {
                    "echo" => {
                        run_echo(&mut writer, node_id.as_str(), &env, &line).await?;
                    }
                    "generate" => {
                        run_generate(&mut writer, node_id.as_str(), &env, &line).await?;
                    }
                    "broadcast" => {
                        run_broadcast(&mut writer, node_id.as_str(), &mut env, &line).await?;
                    }
                    "read" => {
                        run_read(&mut writer, node_id.as_str(), &env, &line).await?;
                    }
                    "topology" => {
                        run_topology(&mut writer, node_id.as_str(), &mut env, &line).await?;
                    }
                    _ => {}
                };

                env.msg_id += 1;
            }
            false => Err(anyhow::anyhow!("Target Node Invalid"))?,
        }
    }

    Ok(())
}

async fn start() -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin); // Lock the stdin for reading

    let mut first_line = String::new();
    reader.read_line(&mut first_line).await?;

    eprintln!("INPUT: {first_line}");
    let init_data = read_json_from_string::<BaseData>(&first_line)?;

    let mut writer = tokio::io::stdout();

    match init_data.body.r#type.as_str() {
        "init" => {
            let (node_id, node_ids) = run_init(&mut writer, &first_line).await?;

            let lines = reader.lines();

            repl(lines, writer, node_id, node_ids).await
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
