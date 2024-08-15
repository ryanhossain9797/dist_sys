mod echo;
mod init;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::{self};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

use echo::*;
use init::*;

async fn print_json_to_stdout<T: Serialize>(data: T) {
    // Serialize the new data to a JSON string
    let json = serde_json::to_string(&data).unwrap();

    // Print the JSON string to stdout
    tokio::io::stdout()
        .write_all(json.as_bytes())
        .await
        .unwrap();
    tokio::io::stdout().write_all(b"\n").await.unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BaseBody {
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BaseData {
    src: String,
    dest: String,
    body: BaseBody,
}

enum AppState {
    Uninitialized,
    Initialized(usize, String, HashSet<String>),
}

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    let mut app_state = AppState::Uninitialized;

    loop {
        if let Some(line) = reader.next_line().await.unwrap() {
            eprintln!("{}", line);
            // Deserialize the JSON string into MyData struct
            let data: BaseData = serde_json::from_str(&line).unwrap();
            match &app_state {
                AppState::Uninitialized => match data.body.r#type.as_str() {
                    "init" => {
                        let (init_response, node_id, node_ids) = get_init_response(&line);

                        app_state = AppState::Initialized(1, node_id, node_ids);

                        print_json_to_stdout(&init_response).await;
                    }
                    _ => {}
                },
                AppState::Initialized(msg_id, id, all_ids) => match id == &data.dest {
                    true => match data.body.r#type.as_str() {
                        "echo" => {
                            let echo_response = get_echo_response(id, *msg_id, &line);

                            app_state =
                                AppState::Initialized(*msg_id + 1, id.clone(), all_ids.clone());

                            print_json_to_stdout(echo_response).await;
                        }
                        _ => {}
                    },
                    false => {}
                },
            }
        }
    }
}
