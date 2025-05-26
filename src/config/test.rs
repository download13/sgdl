use httpmock::prelude::*;

pub fn init_test_server() -> String {
	// Start a lightweight mock server.
	let server = MockServer::start();

	// Create a mock on the server.
	server.mock(|when, then| {
		when.method(GET).path("/u/sgdl-test");
		then
			.status(200)
			.header("content-type", "text/html")
			.body(include_str!("../../testdata/profiles/sgdl-test/index.html"));
	});

	server.mock(|when, then| {
		when.method(GET).path("/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998");
		then
			.status(200)
			.header("content-type", "text/html")
			.body(include_str!("../../testdata/profiles/sgdl-test/tracks/shopping-mall-half-open-Netherlands-207-AM-161001_0998.html"));
	});

	format!("http://localhost:{}", server.port())
}
