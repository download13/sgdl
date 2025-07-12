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
	let response = CLIENT.get(url).send().await?;

	let text = response.text().await?;

	Ok(text)
}

pub async fn stream_bytes(url: String) -> Option<reqwest::Response> {
	CLIENT.get(url).send().await.ok()
}
