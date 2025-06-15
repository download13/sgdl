use lazy_static::lazy_static;
use regex::Regex;

pub struct TrackMetadata {
	pub title: String,
	pub description: String,
}

impl TrackMetadata {
	pub fn from_html(track_page_html: &str) -> Option<Self> {
		let title_matches = TRACK_TITLE_RE.captures(track_page_html.as_str())?;
		let title = String::from(title_matches.get(1)?.as_str());

		let description_matches = TRACK_DESCRIPTION_RE.captures(track_page_html.as_str())?;
		let description = String::from(description_matches.get(1)?.as_str());

		Some(Self { title, description })
	}

	pub async fn add_to_library(&self) -> Result<(), String> {
		// Placeholder for adding track metadata to the library
		// This function should interact with the library system to store the track metadata
		// For now, we just return Ok to indicate success
		Ok(())
	}
}

lazy_static! {
	static ref TRACK_TITLE_RE: Regex =
		Regex::new("<div class=\"jp-title\" aria-label=\"title\">(.+?)</div>").unwrap();
	static ref TRACK_DESCRIPTION_RE: Regex =
		Regex::new("<p style=\"white-space: pre-wrap;\">(.+?)</p>").unwrap();
}
