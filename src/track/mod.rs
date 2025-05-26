use lazy_static::lazy_static;
use regex::Regex;

use crate::common::{fetch_text, PROFILE_PATTERN};
use crate::store::{Store, Track};

pub struct TrackId {
	pub original_url: String,
	pub profile_slug: String,
	pub track_slug: String,
}

lazy_static! {
	static ref TRACK_URL_RE: Regex = Regex::new(
		format!(
			"//(?:www.)?soundgasm.net/u/([{}]+)/([^/]+)",
			PROFILE_PATTERN
		)
		.as_str()
	)
	.unwrap();
}

impl TrackId {
	pub fn new(url: &String) -> Option<TrackId> {
		let matches = TRACK_URL_RE.captures(url.as_str())?;

		let profile_slug = matches.get(1)?.as_str().into();
		let track_slug = matches.get(2)?.as_str().into();

		Some(TrackId {
			original_url: url.to_string(),
			profile_slug,
			track_slug,
		})
	}

	pub async fn get_track_page_html(&self) -> Option<String> {
		let track_page_html = match fetch_text(self.to_url()).await {
			Ok(html) => html,
			Err(err) => {
				println!("Error fetching track page: {}", err);
				return None;
			}
		};

		Some(track_page_html)
	}

	pub fn to_url(&self) -> String {
		format!(
			"//soundgasm.net/u/{}/{}",
			self.profile_slug, self.track_slug
		)
	}
}

#[derive(Debug)]
pub struct TrackDetails {
	pub title: String,
	pub description: String,
	pub sound_id: String,
	pub extension: String,
}

lazy_static! {
	static ref TRACK_TITLE_RE: Regex =
		Regex::new("<div class=\"jp-title\" aria-label=\"title\">(.+?)</div>").unwrap();
	static ref TRACK_DOWNLOAD_RE: Regex =
		Regex::new("//media.soundgasm.net/sounds/([^/]+).([^/]+)").unwrap();
	static ref TRACK_DESCRIPTION_RE: Regex =
		Regex::new("<p style=\"white-space: pre-wrap;\">(.+?)</div>").unwrap();
}

impl TrackDetails {
	pub fn parse_track_details(track_page_html: &String) -> Option<TrackDetails> {
		let title_matches = TRACK_TITLE_RE.captures(track_page_html.as_str())?;
		let title = String::from(title_matches.get(1).unwrap().as_str());

		let description_matches = TRACK_DESCRIPTION_RE.captures(track_page_html.as_str())?;
		let description = String::from(description_matches.get(1).unwrap().as_str());

		let url_matches = TRACK_DOWNLOAD_RE.captures(track_page_html.as_str())?;

		let sound_id = String::from(url_matches.get(1).unwrap().as_str());
		let extension = String::from(url_matches.get(2).unwrap().as_str());

		Some(TrackDetails {
			title,
			description,
			sound_id,
			extension,
		})
	}

	pub fn get_audio_url(&self) -> String {
		format!(
			"https://media.soundgasm.net/sounds/{}.{}",
			self.sound_id, self.extension
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_track_url_info() {
		let track_info =
			TrackId::new(&"//www.soundgasm.net/u/Profess4orCal_/hi-everyone_2".into()).unwrap();
		assert_eq!(track_info.profile_slug, "Profess4orCal_");
		assert_eq!(track_info.track_slug, "hi-everyone_2");

		let track_info =
			TrackId::new(&"//www.soundgasm.net/u/!@#$$^&*()_+/!@#$^&*()_+".into()).unwrap();
		assert_eq!(track_info.profile_slug, "!@#$$^&*()_+");
		assert_eq!(track_info.track_slug, "!@#$^&*()_+");

		let track_info =
			TrackId::new(&"//soundgasm.net/u/Profess4orCal_/hi-everyone_2".into()).unwrap();
		assert_eq!(track_info.profile_slug, "Profess4orCal_");
		assert_eq!(track_info.track_slug, "hi-everyone_2");
	}

	#[test]
	fn test_parse_invalid_track_url() {
		// Not even a URL
		let url = String::from("invalid_url");
		let track_info = TrackId::new(&url);
		assert!(track_info.is_none());

		// Close, but wrong domain
		let track_info = TrackId::new(&"//soundgasm.com/u/Profess4orCal_/hi-everyone_2".into());
		assert!(track_info.is_none());

		// Wrong subdomain
		let track_info = TrackId::new(&"//dfs.soundgasm.net/u/Profess4orCal_/hi-everyone_2".into());
		assert!(track_info.is_none());
	}
}
