mod pointer;

use lazy_static::lazy_static;
use log::debug;
use regex::Regex;

use super::track::{TrackMetadata, TrackPointer, TRACK_SLUG_PATTERN};

pub use pointer::{ProfilePointer, PROFILE_SLUG_PATTERN};

pub struct Profile {
	profile_slug: String,
	tracks: Vec<ProfileTrackListing>,
}

impl Profile {
	pub fn from_html(profile_page_html: &str) -> Option<Self> {
		let Some(sections) = TRACK_SECTION_RE.captures(profile_page_html) else {
			debug!("Failed to capture track sections");
			return None;
		};

		let mut track_listings = Vec::with_capacity(sections.len());

		for section in sections.iter() {
			let section_html = match section {
				Some(s) => s.as_str(),
				None => {
					debug!("Failed to capture section HTML");
					continue;
				}
			};

			let Some(track_url_section) = TRACK_URL_RE.captures(section_html) else {
				debug!("Failed to capture track URL section");
				continue;
			};

			let Some(track_url) = track_url_section.get(1) else {
				debug!("Failed to capture track URL");
				continue;
			};

			let Some(metadata) = TrackMetadata::from_html(section_html) else {
				debug!("Failed to capture track URL");
				continue;
			};

			let Some(track_pointer) = TrackPointer::from_url(track_url.as_str()) else {
				continue; // Skip if track URL is invalid
			};

			track_listings.push(ProfileTrackListing {
				pointer: track_pointer,
				metadata,
			});
		}

		if track_listings.is_empty() {
			debug!("No valid track listings found in profile page HTML");
			return None;
		}

		Some(Self {
			profile_slug: track_listings.get(0).unwrap().pointer.profile_slug.clone(),
			tracks: track_listings,
		})
	}

	pub fn new(slug: String, tracks: Vec<ProfileTrackListing>) -> Self {
		Self {
			profile_slug: slug,
			tracks,
		}
	}
}

pub struct ProfileTrackListing {
	pub(super) pointer: TrackPointer,
	pub(super) metadata: TrackMetadata,
}

lazy_static! {
	static ref TRACK_SECTION_RE: Regex = Regex::new(
		format!(
			"<div class=\"sound-details\">([{}]+?)</div>",
			TRACK_SLUG_PATTERN
		)
		.as_str()
	)
	.unwrap();
	static ref TRACK_URL_RE: Regex = Regex::new("<a href=\"(.+?)\"").unwrap();
	static ref TRACK_TITLE_RE: Regex = Regex::new("<a href=\"(?:.+?)\">(.+?)</a>").unwrap();
	static ref TRACK_DESCRIPTION_RE: Regex =
		Regex::new("<span class=\"soundDescription\">(.+?)</span>").unwrap();
}
