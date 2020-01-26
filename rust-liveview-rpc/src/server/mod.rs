use crate::prelude::*;
use async_std::{
    future::Future,
    pin::Pin,
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

impl Future for Server {
    type Output = Result<(), <Self as Service<Request<()>>>::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll_ready(cx)
    }
}
