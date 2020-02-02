use crate::prelude::*;
use std::{
    future::Future,
    task::{
        Context,
        Poll,
    },
};

#[derive(Debug)]
pub struct Server {
    message: String,
}

impl Server {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self { message: s.into() }
    }
}

impl Service<Request<()>> for Server {
    type Response = Response<Bytes>;
    type Error = http::Error;
    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<()>) -> Self::Future {
        let message = self.message.clone();
        Box::pin(async { Response::builder().body(Bytes::from(message)) })
    }
}
