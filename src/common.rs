use lazy_static::lazy_static;
use reqwest::{Client, Error};

lazy_static! {
	static ref CLIENT: Client = Client::builder().user_agent(USER_AGENT).build().unwrap();
}

pub async fn fetch_text(url: String) -> Result<String, Error> {
	let response = CLIENT.get(url).send().await?;

	let text = response.text().await?;

	Ok(text)
}

pub const USER_AGENT: &str = "sgdl/0.1 (testing)";
