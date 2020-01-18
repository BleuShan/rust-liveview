// edition:2018

use async_std::{
    io,
    prelude::*,
    sync::channel,
    task,
};
use rust_liveview::runtime;
use std::time::Duration;

#[runtime(executor_entrypoint = "task::block_on")]
async fn main() -> io::Result<()> {
    let (tx, mut rx) = channel(1);
    task::spawn(async move {
        tx.send("Hello, ").await;
        task::sleep(Duration::from_millis(100)).await;
        tx.send("World!").await;
        task::sleep(Duration::from_millis(100)).await;
    });
    while let Some(message) = rx.next().await {
        io::stdout().write_all(message.as_bytes()).await?;
    }
    io::stdout().write_all(b"\n").await
}
