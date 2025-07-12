use lazy_static::lazy_static;
use regex::Regex;

use super::sound_pointer::TrackSoundPointer;
use crate::media_sources::soundgasm::{
	profile::PROFILE_SLUG_PATTERN, track::TrackMetadata, SoundgasmAudioTrackRow,
};

pub const TRACK_SLUG_PATTERN: &str = "a-zA-Z0-9_-";

#[derive(Debug, Clone)]
pub struct TrackPointer {
	pub profile_slug: String,
	pub track_slug: String,
}

impl TrackPointer {
	pub fn from_url(url: &str) -> Result<Self, String> {
		let captures = TRACK_URL_RE.captures(url);

		match captures {
			Some(caps) => {
				let profile_slug = caps
					.get(1)
					.ok_or(format!("Invalid profile slug in URL: {}", url))?;
				let track_slug = caps
					.get(2)
					.ok_or(format!("Invalid track slug in URL: {}", url))?;

				Ok(Self {
					profile_slug: profile_slug.as_str().to_string(),
					track_slug: track_slug.as_str().to_string(),
				})
			}
			None => Err("Unable to parse track pointer from URL".to_string()),
		}
	}

	pub fn to_url(&self) -> String {
		format!(
			"https://soundgasm.net/u/{}/{}",
			self.profile_slug, self.track_slug
		)
	}

	pub async fn fetch_track_page(&self) -> Option<(TrackMetadata, TrackSoundPointer)> {
		let response = reqwest::get(self.to_url()).await.ok()?;
		let html = response.text().await.ok()?;

		let meta = TrackMetadata::from_html(&html)?;
		let sound = TrackSoundPointer::from_html(&html)?;

		Some((meta, sound))
	}
}

impl From<SoundgasmAudioTrackRow> for TrackPointer {
	fn from(row: SoundgasmAudioTrackRow) -> Self {
		Self {
			profile_slug: row.profile_slug,
			track_slug: row.track_slug,
		}
	}
}

lazy_static! {
	static ref TRACK_URL_RE: Regex = Regex::new(
		format!(
			"//(?:www.)?soundgasm.net/u/([{}]+)/([{}]+)/?",
			PROFILE_SLUG_PATTERN, TRACK_SLUG_PATTERN
		)
		.as_str()
	)
	.unwrap();
}

#[cfg(test)]
mod tests {
	use super::TrackPointer;

	#[test]
	fn test_parse_track_pointer() {
		// With subdomain
		let track_info = TrackPointer::from_url(
			"https://www.soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998",
		)
		.unwrap();
		assert_eq!(track_info.profile_slug, "sgdl-test");
		assert_eq!(
			track_info.track_slug,
			"shopping-mall-half-open-Netherlands-207-AM-161001_0998"
		);

		// Without subdomain and schema
		let track_info = TrackPointer::from_url(
			"//soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998",
		)
		.unwrap();
		assert_eq!(track_info.profile_slug, "sgdl-test");
		assert_eq!(
			track_info.track_slug,
			"shopping-mall-half-open-Netherlands-207-AM-161001_0998"
		);

		// With trailing slash
		let track_info = TrackPointer::from_url(
			"//soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998/",
		)
		.unwrap();
		assert_eq!(track_info.profile_slug, "sgdl-test");
		assert_eq!(
			track_info.track_slug,
			"shopping-mall-half-open-Netherlands-207-AM-161001_0998"
		);
	}

	#[test]
	fn test_parse_invalid_track_pointer() {
		// Not even a URL
		let track_info = TrackPointer::from_url("invalid_url");
		assert!(track_info.is_err());

		// Close, but wrong tld
		let track_info = TrackPointer::from_url(
			"//soundgasm.com/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998",
		);
		assert!(track_info.is_err());

		// Wrong subdomain
		let track_info = TrackPointer::from_url(
			"//dfs.soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998",
		);
		assert!(track_info.is_err());
	}
}
