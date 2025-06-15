use lazy_static::lazy_static;
use regex::Regex;

use crate::media_sources::soundgasm::{
	profile::ProfilePointer,
	track::{metadata::TrackMetadata, TRACK_SLUG_PATTERN},
};

// No difference in slug pattern for tracks and profiles as far as I can tell
const PROFILE_SLUG_PATTERN: &str = TRACK_SLUG_PATTERN;

pub struct TrackPointer {
	pub profile: ProfilePointer,
	pub slug: String,
}

impl TrackPointer {
	pub fn from_url(url: &str) -> Option<Self> {
		TRACK_URL_RE.captures(url).map(|caps| {
			let profile_slug = caps.get(1)?;
			let track_slug = caps.get(2)?;

			Some(Self {
				profile: ProfilePointer {
					slug: profile_slug.as_str().to_string(),
				},
				slug: track_slug.as_str().to_string(),
			})
		})?
	}

	pub fn get_profile(&self) -> &ProfilePointer {
		&self.profile
	}

	pub fn get_slug(&self) -> &String {
		&self.slug
	}

	pub fn to_url(&self) -> String {
		format!(
			"https://soundgasm.net/u/{}/{}",
			self.profile.slug, self.slug
		)
	}

	pub async fn fetch_metadata(&self) -> Option<TrackMetadata> {
		let response = reqwest::get(self.to_url()).await.ok()?;
		let html = response.text().await.ok()?;

		TrackMetadata::from_html(&html)
	}
}

lazy_static! {
	static ref TRACK_URL_RE: Regex = Regex::new(
		format!(
			"//(?:www.)?soundgasm.net/u/([{}]+?)/([{}]+?)/",
			PROFILE_SLUG_PATTERN, TRACK_SLUG_PATTERN
		)
		.as_str()
	)
	.unwrap();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_track_url_info() {
		let track_info = TrackPointer::from_url(
			&"//www.soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998",
		)
		.unwrap();
		assert_eq!(track_info.profile.slug, "sgdl-test");
		assert_eq!(
			track_info.slug,
			"shopping-mall-half-open-Netherlands-207-AM-161001_0998"
		);

		// Without schema
		let track_info =
			TrackPointer::from_url(&"//www.soundgasm.net/u/!@#$$^&*()_+/!@#$^&*()_+").unwrap();
		assert_eq!(track_info.profile.slug, "!@#$$^&*()_+");
		assert_eq!(track_info.slug, "!@#$^&*()_+");

		// Without schema or subdomain
		let track_info = TrackPointer::from_url(
			&"//soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998",
		)
		.unwrap();
		assert_eq!(track_info.profile.slug, "/sgdl-test");
		assert_eq!(
			track_info.slug,
			"shopping-mall-half-open-Netherlands-207-AM-161001_0998"
		);
	}

	#[test]
	fn test_parse_invalid_track_url() {
		// Not even a URL
		let track_info = TrackPointer::from_url(&"invalid_url");
		assert!(track_info.is_none());

		// Close, but wrong tld
		let track_info = TrackPointer::from_url(
			&"//soundgasm.com/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998",
		);
		assert!(track_info.is_none());

		// Wrong subdomain
		let track_info = TrackPointer::from_url(
			&"//dfs.soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998",
		);
		assert!(track_info.is_none());
	}
}
