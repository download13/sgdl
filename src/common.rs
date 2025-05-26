use lazy_static::lazy_static;
use reqwest::{Client, Error};

lazy_static! {
	static ref CLIENT: Client = {
		Client::builder()
			.user_agent("sgdl/0.1 (testing)")
			.build()
			.unwrap()
	};
}

pub async fn fetch_text(url: String) -> Result<String, Error> {
	CLIENT.get(url).send().await?.text().await
}

pub async fn stream_bytes(url: String) -> Result<reqwest::Response, Error> {
	CLIENT.get(url).send().await
}

pub const PROFILE_PATTERN: &str = "a-zA-Z0-9_-";
