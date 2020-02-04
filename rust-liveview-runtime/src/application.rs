#![allow(clippy::new_ret_no_self)]

use crate::tls::TlsBuilder;
use async_std::{
    io,
    net::{
        TcpListener,
        ToSocketAddrs,
    },
};
use rust_liveview_common::Builder;

#[derive(Builder, Debug)]
#[builder(setter(strip_option, into), pattern = "owned")]
pub struct Application {
    tls: Option<TlsBuilder>,
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }

    pub async fn bind<A>(&self, addr: A) -> io::Result<TcpListener>
    where
        A: ToSocketAddrs,
    {
        Ok(TcpListener::bind(addr).await?)
    }
}
