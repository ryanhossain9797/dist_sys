use serde::Serialize;
use tokio::io::{AsyncWriteExt, BufReader, Lines, Stdin};

pub async fn print_json_to_stdout<T: Serialize>(data: T) {
    let json = serde_json::to_string(&data).unwrap();

    if !json.contains("src") {
        eprintln!("src missing {json}");
        panic!();
    }

    tokio::io::stdout()
        .write_all(json.as_bytes())
        .await
        .unwrap();
    tokio::io::stdout().write_all(b"\n").await.unwrap();
}

pub async fn read_json_from_stdin<T: for<'a> serde::Deserialize<'a>>(
    reader: &mut Lines<BufReader<Stdin>>,
) -> (T, String) {
    let init_line = reader.next_line().await.unwrap().unwrap();
    let init_data = serde_json::from_str::<T>(&init_line).unwrap();

    (init_data, init_line)
}
