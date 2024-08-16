use serde::Serialize;
use tokio::io::{AsyncWriteExt, BufReader, Lines, Stdin};

pub async fn print_json_to_stdout<T: Serialize>(data: T) -> anyhow::Result<()> {
    let json = serde_json::to_string(&data)?;

    if !json.contains("src") {
        eprintln!("src missing {json}");
        panic!("src missing {json}");
    }

    tokio::io::stdout().write_all(json.as_bytes()).await?;
    tokio::io::stdout().write_all(b"\n").await?;
    Ok(())
}

pub async fn read_json_from_stdin<T: for<'a> serde::Deserialize<'a>>(
    reader: &mut Lines<BufReader<Stdin>>,
) -> anyhow::Result<(T, String)> {
    let init_line = reader
        .next_line()
        .await?
        .ok_or(anyhow::anyhow!("Empty line"))?;
    let init_data = serde_json::from_str::<T>(&init_line)?;

    Ok((init_data, init_line))
}
