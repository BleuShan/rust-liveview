use super::*;

struct ServerTests(TestBackend);

impl Default for ServerTests {
    fn default() -> Self {
        Self(task::block_on(TestBackend::new("Test")).expect("Failed to spawn server"))
    }
}

#[session]
impl ServerTests {
    #[fact]
    #[runtime(executor_entrypoint = "task::block_on")]
    async fn server_should_respond_to_request(self) {
        se
    }
}
