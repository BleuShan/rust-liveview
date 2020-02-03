pub use async_tls::{
    TlsAcceptor,
    TlsConnector,
};
use rustls::{
    internal::pemfile::{
        certs,
        rsa_private_keys,
    },
    ClientConfig,
    NoClientAuth,
    ServerConfig,
};
use std::{
    fs,
    io::{
        self,
        BufReader,
        Cursor,
    },
    ops::Try,
    path::PathBuf,
    sync::Arc,
};

#[derive(Default, Debug)]
pub struct TlsBuilder {
    cert_path: Option<PathBuf>,
    key_path: Option<PathBuf>,
}

impl TlsBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn build_acceptor(&self) -> io::Result<TlsAcceptor> {
        if self.cert_path.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Missing cert_path.",
            ));
        }

        if self.key_path.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Missing key_path.",
            ));
        }

        let config = ServerConfig::new(NoClientAuth::new());
        let _key = self
            .key_path
            .as_ref()
            .into_result()
            .map_err(|_| io::Error::from(io::ErrorKind::NotFound))
            .map(|path| {
                let mut file = BufReader::new(fs::File::open(path)?);
                rsa_private_keys(&mut file)
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid key"))
            })?;
        let _certs = self
            .cert_path
            .as_ref()
            .into_result()
            .map_err(|_| io::Error::from(io::ErrorKind::NotFound))
            .map(|path| {
                let mut file = BufReader::new(fs::File::open(path)?);
                certs(&mut file)
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid key"))
            })?;

        Ok(TlsAcceptor::from(Arc::new(config)))
    }

    pub fn build_connector(&self) -> io::Result<TlsConnector> {
        let mut config = ClientConfig::new();
        config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        if let Some(path) = &self.cert_path {
            let file = fs::read(path)?;
            config
                .root_store
                .add_pem_file(&mut Cursor::new(file))
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid cert."))?;
        }

        Ok(TlsConnector::from(Arc::new(config)))
    }
}
