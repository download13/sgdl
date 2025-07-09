use lazy_static::lazy_static;
use regex::Regex;

use super::super::track::TrackPointer;
use crate::{common::fetch_text, media_sources::soundgasm::profile::Profile};

pub const PROFILE_SLUG_PATTERN: &str = "a-zA-Z0-9_-";

#[derive(Debug, Clone)]
pub struct ProfilePointer {
	slug: String,
}

impl ProfilePointer {
	pub fn from_slug(maybe_slug: &str) -> Option<Self> {
		let profile_matches = PROFILE_SLUG_RE.captures(maybe_slug)?;
		let slug = profile_matches.get(1)?.as_str().to_string();

		Some(Self { slug })
	}

	pub fn from_url(url: &str) -> Option<Self> {
		let profile_matches = PROFILE_URL_RE.captures(url)?;
		let slug = profile_matches.get(1)?.as_str().to_string();

		Some(Self { slug })
	}

	pub fn get_url(&self) -> String {
		format!("https://soundgasm.net/u/{}", self.slug)
	}

	pub async fn fetch_profile(&self) -> Option<Profile> {
		let profile_html = fetch_text(self.get_url()).await.ok()?;

		Some(Profile::from_html(&profile_html)?)
	}
}

impl From<TrackPointer> for ProfilePointer {
	fn from(track_pointer: TrackPointer) -> Self {
		Self {
			slug: track_pointer.profile_slug.clone(),
		}
	}
}

lazy_static! {
	static ref PROFILE_URL_RE: Regex =
		Regex::new(format!("//(?:www.)?soundgasm.net/u/([{}]+)/?", PROFILE_SLUG_PATTERN).as_str())
			.unwrap();
	static ref PROFILE_SLUG_RE: Regex =
		Regex::new(format!("([{}]+)", PROFILE_SLUG_PATTERN).as_str()).unwrap();
}

#[cfg(test)]
mod tests {
	use super::ProfilePointer;

	#[test]
	fn test_parse_profile_pointer_from_url() {
		// With subdomain
		let pointer = ProfilePointer::from_url("https://www.soundgasm.net/u/sgdl-test").unwrap();
		assert_eq!(pointer.slug, "sgdl-test");

		// Without subdomain and schema
		let pointer = ProfilePointer::from_url("//soundgasm.net/u/sgdl-test").unwrap();
		assert_eq!(pointer.slug, "sgdl-test");

		// With trailing slash
		let pointer = ProfilePointer::from_url("//soundgasm.net/u/sgdl-test/").unwrap();
		assert_eq!(pointer.slug, "sgdl-test");
	}

	#[test]
	fn test_parse_invalid_profile_pointer() {
		// Not even a URL
		let pointer = ProfilePointer::from_url("invalid_url");
		assert!(pointer.is_none());

		// Close, but wrong tld
		let pointer = ProfilePointer::from_url("//soundgasm.com/u/sgdl-test");
		assert!(pointer.is_none());

		// Wrong subdomain
		let pointer = ProfilePointer::from_url("//dfs.soundgasm.net/u/sgdl-test/");
		assert!(pointer.is_none());
	}
}
