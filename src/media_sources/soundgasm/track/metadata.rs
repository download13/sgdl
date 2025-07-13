use lazy_static::lazy_static;
use regex::Regex;

use crate::media_types::MediaMetadata;

use super::SoundgasmAudioTrackRow;

#[derive(Clone, Debug)]
pub struct TrackMetadata {
	pub title: String,
	pub description: String,
}

impl TrackMetadata {
	pub fn from_html(track_page_html: &str) -> Option<Self> {
		let title_matches = TRACK_TITLE_RE.captures(track_page_html)?;
		let title = String::from(title_matches.get(1)?.as_str());

		let description_matches = TRACK_DESCRIPTION_RE.captures(track_page_html)?;
		let description = String::from(description_matches.get(1)?.as_str());

		Some(Self { title, description })
	}
}

impl From<SoundgasmAudioTrackRow> for TrackMetadata {
	fn from(row: SoundgasmAudioTrackRow) -> Self {
		Self {
			title: row.title,
			description: row.description,
		}
	}
}

impl MediaMetadata for TrackMetadata {
	async fn get_title(&self) -> String {
		self.title.clone()
	}

	async fn get_description(&self) -> String {
		self.description.clone()
	}
}

lazy_static! {
	static ref TRACK_TITLE_RE: Regex =
		Regex::new("<div class=\"jp-title\" aria-label=\"title\">(.+?)</div>").unwrap();
	static ref TRACK_DESCRIPTION_RE: Regex =
		Regex::new("<p style=\"white-space: pre-wrap;\">(.+?)</p>").unwrap();
}
