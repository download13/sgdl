use reqwest;

pub async fn fetch_text(url: String) -> Option<String> {
	reqwest::get(url).await.ok()?.text().await.ok()
}
