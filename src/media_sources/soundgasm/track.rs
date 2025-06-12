use diesel::{deserialize::FromSqlRow, expression::AsExpression};
use lazy_static::lazy_static;
use regex::Regex;

use crate::common::{fetch_text, PROFILE_PATTERN};

pub struct SoundgasmTrackPointer {
	profile: SoundgasmProfilePointer,
	slug: String,
}

pub impl SoundgasmTrackPointer {
	pub fn try_parse(track_id_or_url: &String) -> Option<SoundgasmTrackPointer> {
		let profile = SoundgasmProfilePointer::parse(track_id_or_url)?;
		let track_slug = Self::parse_track_slug(track_id_or_url)?;

		if track_slug.is_empty() {
			return None;
		}

		Some(SoundgasmTrackPointer {
			profile,
			slug: track_slug,
		})
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

	#[cfg(not(test))]
	pub fn get_audio_url(&self) -> String {
		format!(
			"https://media.soundgasm.net/sounds/{}.{}",
			self.sound_id, self.extension
		)
	}

	#[cfg(test)]
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
		let track_info = TrackId::new(
			&"//www.soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998"
				.into(),
		)
		.unwrap();
		assert_eq!(track_info.profile_slug, "sgdl-test");
		assert_eq!(
			track_info.track_slug,
			"shopping-mall-half-open-Netherlands-207-AM-161001_0998"
		);

		// Without schema
		let track_info =
			TrackId::new(&"//www.soundgasm.net/u/!@#$$^&*()_+/!@#$^&*()_+".into()).unwrap();
		assert_eq!(track_info.profile_slug, "!@#$$^&*()_+");
		assert_eq!(track_info.track_slug, "!@#$^&*()_+");

		// Without schema or subdomain
		let track_info = TrackId::new(
			&"//soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998".into(),
		)
		.unwrap();
		assert_eq!(track_info.profile_slug, "/sgdl-test");
		assert_eq!(
			track_info.track_slug,
			"shopping-mall-half-open-Netherlands-207-AM-161001_0998"
		);
	}

	#[test]
	fn test_parse_invalid_track_url() {
		// Not even a URL
		let url = String::from("invalid_url");
		let track_info = TrackId::new(&url);
		assert!(track_info.is_none());

		// Close, but wrong tld
		let track_info = TrackId::new(
			&"//soundgasm.com/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998".into(),
		);
		assert!(track_info.is_none());

		// Wrong subdomain
		let track_info = TrackId::new(
			&"//dfs.soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998"
				.into(),
		);
		assert!(track_info.is_none());
	}
}
