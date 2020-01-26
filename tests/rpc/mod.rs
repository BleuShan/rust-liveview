use super::*;
use async_std::{
    io,
    task,
};
use rust_liveview::{
    rpc::{
        prelude::*,
        Server,
    },
    runtime,
};
use rust_liveview_common::http;

mod server;

struct TestBackend(Server);

impl TestBackend {
    async fn new<S: Into<String>>(s: S) -> Result<Self, <Server as Service<Request<()>>>::Error> {
        let inner = Server::new(s);
        inner.await?;
        Ok(Self(inner))
    }
}
