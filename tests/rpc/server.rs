use super::*;

#[derive(Deref, DerefMut, Debug)]
struct ServerTests(TestBackend);

impl Default for ServerTests {
    fn default() -> Self {
        Self(task::block_on(TestBackend::new("Test")).expect("Failed to spawn server"))
    }
}

#[session]
impl ServerTests {
    #[fact]
    fn server_should_respond_to_request(mut self) {
        task::block_on(self.call(Request::default()))
            .map(|response| response.body().clone())
            .should()
            .be_ok()
            .and_should()
            .yield_the_item(Bytes::from("Test"));
    }
}
