use super::*;

use ::async_std::{
    prelude::*,
    sync::channel,
    task,
};
use rust_liveview::runtime;
use std::time::Duration;

#[runtime::test(executor_entrypoint = "task::block_on", test)]
async fn runtime_test_should_add_the_test_attribute() {
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

#[runtime::test(executor_entrypoint = "task::block_on")]
#[test]
async fn runtime_test_should_work_with_test_attribute() {
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

#[runtime::test(executor_entrypoint = "task::block_on")]
#[fact]
async fn runtime_test_should_work_with_fluid_fact_attribute() {
    let (tx, mut rx) = channel(1);
    task::spawn(async move {
        tx.send("Hello, ").await;
        task::sleep(Duration::from_millis(100)).await;
        tx.send("World!").await;
        task::sleep(Duration::from_millis(100)).await;
    });
    while let Some(message) = rx.next().await {
        message.should().not().be_empty();
    }
}

#[runtime::test(executor_entrypoint = "task::block_on")]
#[theory]
#[case(1, 1)]
#[case(2, 2)]
async fn runtime_test_should_work_with_fluid_theory_attribute(a: u64, b: u64) {
    let (tx, mut rx) = channel(1);
    task::spawn(async move {
        task::sleep(Duration::from_millis(100)).await;
        tx.send(a + b).await
    });
    while let Some(message) = rx.next().await {
        message.should().be_equal_to(a + b);
    }
}
