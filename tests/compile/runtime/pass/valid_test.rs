// edition:2018

#![allow(unused_imports)]
use async_std::{
    io,
    prelude::*,
    sync::channel,
    task,
};
use rust_liveview::runtime;
use std::time::Duration;

fn main() {}

#[runtime(executor_entrypoint = "task::block_on", test)]
async fn test() -> io::Result<()> {
    let (tx, mut rx) = channel(1);
    task::spawn(async move {
        tx.send("Hello, ").await;
        task::sleep(Duration::from_millis(100)).await;
        tx.send("World!").await;
        task::sleep(Duration::from_millis(100)).await;
    });
    while let Some(message) = rx.next().await {
        assert!(!message.is_empty());
    }
}
