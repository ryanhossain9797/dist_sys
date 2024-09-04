/*
{"src":"c1","dest":"n3","body":{"type":"init","msg_id":1,"node_id":"n3","node_ids":["n1", "n2","n3"]}}

{"src":"c1","dest":"n3","body":{"type":"echo","msg_id":1,"echo":"Please echo 35"}}

{"src":"c1","dest":"n3","body":{"type":"generate","msg_id":1}}
 */
mod init;
mod types;
mod utils;
mod workloads;

use std::collections::HashSet;
use std::io::{self, BufRead, StdinLock};

use init::*;
use types::base::BaseData;
use utils::read_json_from_string;

use workloads::broadcast::run_broadcast;
use workloads::echo::run_echo;
use workloads::generate::run_generate;
use workloads::read::run_read;
use workloads::topology::run_topology;

struct Environment {
    msg_id: usize,
    received: Vec<usize>,
}

pub fn repl(
    handle: StdinLock<'static>,
    node_id: String,
    _node_ids: HashSet<String>,
) -> anyhow::Result<()> {
    let mut env = Environment {
        msg_id: 1,
        received: Vec::new(),
    };

    for line in handle.lines() {
        let line = line?;
        let data = read_json_from_string::<BaseData>(&line)?;
        eprintln!("INPUT: {line}");
        match node_id == data.dest {
            true => {
                match data.body.r#type.as_str() {
                    "echo" => {
                        run_echo(node_id.as_str(), &env, &line)?;
                    }
                    "generate" => {
                        run_generate(node_id.as_str(), &env, &line)?;
                    }
                    "broadcast" => {
                        run_broadcast(node_id.as_str(), &mut env, &line)?;
                    }
                    "read" => {
                        run_read(node_id.as_str(), &env, &line)?;
                    }
                    "topology" => {
                        run_topology(node_id.as_str(), &env, &line)?;
                    }
                    _ => {}
                };

                env = Environment {
                    msg_id: env.msg_id + 1,
                    ..env
                };
            }
            false => Err(anyhow::anyhow!("Target Node Invalid"))?,
        }
    }

    Ok(())
}

fn start() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock(); // Lock the stdin for reading

    let mut first_line = String::new();
    handle.read_line(&mut first_line)?;
    eprintln!("INPUT: {first_line}");
    let init_data = read_json_from_string::<BaseData>(&first_line)?;

    match init_data.body.r#type.as_str() {
        "init" => {
            let (node_id, node_ids) = run_init(&first_line)?;

            repl(handle, node_id, node_ids)
        }
        _ => Err(anyhow::anyhow!("Not Init")),
    }
}

fn main() {
    let failure = start();

    match failure {
        Ok(()) => {
            panic!("Unreacahble")
        }
        Err(err) => {
            eprint!("{err}")
        }
    }
}
