use super::*;
use rust_liveview::runtime;
use std::time::Duration;
use tokio::{
    stream::StreamExt,
    sync::mpsc::channel,
    task,
    time::delay_for,
};

#[runtime::test(executor = "tokio_threaded")]
async fn runtime_test_should_add_the_test_attribute() {
    let (mut tx, mut rx) = channel(1);
    task::spawn(async move {
        tx.send("Hello, ").await.should().be_ok();
        delay_for(Duration::from_millis(100)).await;
        tx.send("World!").await.should().be_ok();
        delay_for(Duration::from_millis(100)).await;
    });
    while let Some(message) = rx.recv().await {
        assert!(!message.is_empty());
    }
}

#[runtime::test(executor = "tokio_threaded", kind = "fact")]
async fn runtime_test_should_work_with_fluid_fact_attribute() {
    let (mut tx, mut rx) = channel(1);
    task::spawn(async move {
        tx.send("Hello, ").await.should().be_ok();
        delay_for(Duration::from_millis(100)).await;
        tx.send("World!").await.should().be_ok();
        delay_for(Duration::from_millis(100)).await;
    });
    while let Some(message) = rx.next().await {
        message.should().not().be_empty();
    }
}

#[runtime::test(executor = "tokio_threaded", kind = "theory")]
#[case(1, 1)]
#[case(2, 2)]
async fn runtime_test_should_work_with_fluid_theory_attribute(a: u64, b: u64) {
    let (mut tx, mut rx) = channel(1);
    task::spawn(async move {
        delay_for(Duration::from_millis(100)).await;
        tx.send(a + b).await
    });
    while let Some(message) = rx.next().await {
        message.should().be_equal_to(a + b);
    }
}
