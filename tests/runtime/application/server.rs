use super::*;

use runtime::{
    Application,
    TlsBuilder,
};

fn configure() -> Application {
    Application::new()
        .tls(
            TlsBuilder::new()
                .cert_path("certs/locahost.pem")
                .key_path("certs/locahost-key.pem"),
        )
        .build()
        .expect("Failed to create server")
}

#[runtime::test(async_std, fact)]
async fn a_server_should_be_able_to_handle_requests() {
    let _server = configure();
}
