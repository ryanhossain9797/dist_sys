use serde::Serialize;
use tokio::io::{AsyncWriteExt, BufReader, Lines, Stdin};

pub fn print_json_to_stdout<T: Serialize>(data: T) -> anyhow::Result<()> {
    let json = serde_json::to_string(&data)?;

    eprintln!("OUTPUT: {json}");

    println!("{json}");
    Ok(())
}

pub fn read_json_from_string<T: for<'a> serde::Deserialize<'a>>(line: &str) -> anyhow::Result<T> {
    let data = serde_json::from_str::<T>(line)?;

    Ok(data)
}
