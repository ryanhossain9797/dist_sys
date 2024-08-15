use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::{self};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

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
struct EchoBody {
    r#type: String,
    msg_id: usize,
    in_reply_to: Option<usize>,
    echo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EchoData {
    src: String,
    dest: String,
    body: EchoBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InitBody {
    r#type: String,
    msg_id: usize,
    node_id: String,
    node_ids: HashSet<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InitData {
    src: String,
    dest: String,
    body: InitBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InitResponseBody {
    r#type: String,
    in_reply_to: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InitResponseData {
    src: String,
    dest: String,
    body: InitResponseBody,
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
    Initialized(String, HashSet<String>),
}

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    let mut app_state = AppState::Uninitialized;
    let mut msg_id = 1;

    loop {
        if let Some(line) = reader.next_line().await.unwrap() {
            eprintln!("{}", line);
            // Deserialize the JSON string into MyData struct
            let data: BaseData = serde_json::from_str(&line).unwrap();

            match &app_state {
                AppState::Uninitialized => match data.body.r#type.as_str() {
                    "init" => {
                        let init_data: InitData = serde_json::from_str(&line).unwrap();
                        let init_response = InitResponseData {
                            src: init_data.body.node_id.clone(),
                            dest: init_data.src,
                            body: InitResponseBody {
                                r#type: "init_ok".to_string(),
                                in_reply_to: init_data.body.msg_id,
                            },
                        };

                        app_state =
                            AppState::Initialized(init_data.body.node_id, init_data.body.node_ids);

                        print_json_to_stdout(&init_response).await;
                    }
                    _ => {}
                },
                AppState::Initialized(id, _) => match id == &data.dest {
                    true => match data.body.r#type.as_str() {
                        "echo" => {
                            let echo_data: EchoData = serde_json::from_str(&line).unwrap();
                            let echo_response: EchoData = EchoData {
                                src: id.clone(),
                                dest: echo_data.src,
                                body: EchoBody {
                                    r#type: "echo_ok".to_string(),
                                    msg_id: msg_id,
                                    in_reply_to: Some(echo_data.body.msg_id),
                                    echo: echo_data.body.echo,
                                },
                            };
                            msg_id = msg_id + 1;
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
