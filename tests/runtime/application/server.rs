use super::*;

use runtime::Application;

async fn build_server() -> Application {
    Application::new().build().await
}

#[runtime::test(tokio_threaded, fact)]
async fn a_server_should_be_able_to_handle_requests() {
    let _server = build_server();
}
