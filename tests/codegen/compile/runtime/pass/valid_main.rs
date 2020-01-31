use async_std::{
    io,
    prelude::*,
    sync::channel,
    task,
};
use rust_liveview::runtime;
use std::time::Duration;

#[runtime::main(executor = "async_std")]
async fn main() -> io::Result<()> {
    let (tx, mut rx) = channel(1);
    let mut buffer: Vec<u8> = Default::default();
    task::spawn(async move {
        tx.send("Hello, ").await;
        task::sleep(Duration::from_millis(100)).await;
        tx.send("World!").await;
        task::sleep(Duration::from_millis(100)).await;
    });
    while let Some(message) = rx.next().await {
        buffer.write_all(message.as_bytes()).await?;
    }
    buffer.write_all(b"\n").await?;
    assert_eq!(String::from_utf8_lossy(&buffer), "Hello, World!\n");
    Ok(())
}
