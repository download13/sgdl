use lazy_static::lazy_static;
use regex::Regex;

use crate::media_sources::soundgasm::TrackPointer;

pub const PROFILE_SLUG_PATTERN: &str = "a-zA-Z0-9_-";

#[derive(Clone)]
pub struct ProfilePointer {
	pub slug: String,
}

impl ProfilePointer {
	pub fn new(slug: &str) -> Self {
		Self {
			slug: slug.to_string(),
		}
	}
}

impl ProfilePointer {
	pub fn from_url(url: &str) -> Option<Self> {
		let profile_slug = Self::parse_profile_slug(url)?;
		if profile_slug.is_empty() {
			return None;
		}

		Some(ProfilePointer { slug: profile_slug })
	}

	pub fn from_track_pointer(track_pointer: &TrackPointer) -> Self {
		track_pointer.profile.clone()
	}

	pub fn parse_profile_slug(profile_id_or_url: &String) -> Option<String> {
		let profile_slug = PROFILE_URL_RE.captures(profile_id_or_url)?;

		let profile_slug = profile_slug.get(1)?.as_str();
		Some(profile_slug.to_string())
	}
}

lazy_static! {
	static ref PROFILE_URL_RE: Regex = Regex::new(
		format!(
			"//(?:www.)?soundgasm.net/u/([{}]+?)/?",
			PROFILE_SLUG_PATTERN
		)
		.as_str()
	)
	.unwrap();
}
