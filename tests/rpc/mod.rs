use super::*;
use async_std::task;
use rust_liveview::rpc::{
    prelude::*,
    Server,
};
use rust_liveview_common::{
    http::Request,
    Deref,
    DerefMut,
};

mod server;

#[derive(Deref, DerefMut, Debug)]
struct TestBackend(Server);

impl TestBackend {
    async fn new<S: Into<String>>(s: S) -> Result<Self, <Server as Service<Request<()>>>::Error> {
        Ok(Self(Server::new(s)))
    }
}
