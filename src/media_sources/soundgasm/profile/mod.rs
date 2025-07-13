mod pointer;

use lazy_static::lazy_static;
use log::debug;
use regex::Regex;

use super::track::{SoundgasmAudioTrack, TrackMetadata, TrackPointer};

pub use pointer::{ProfilePointer, PROFILE_SLUG_PATTERN};

pub struct Profile {
	pub slug: String,
	pub tracks: Vec<ProfileTrackListing>,
}

impl Profile {
	pub fn from_html(profile_page_html: &str) -> Result<Self, String> {
		let sections_iter = TRACK_SECTION_RE.captures_iter(profile_page_html);

		let mut tracks = Vec::new();

		for section_capture in sections_iter {
			let (_, [section_html]) = section_capture.extract();

			let Some(track_url_and_title) = TRACK_URL_AND_TITLE_RE.captures(section_html) else {
				debug!("Failed to capture track URL and title");
				continue;
			};

			let (_, [url, title]) = track_url_and_title.extract();

			let Ok(pointer) = TrackPointer::from_url(url) else {
				continue; // Skip if track URL is invalid
			};

			let Some(description_capture) = TRACK_DESCRIPTION_RE.captures(section_html) else {
				debug!("Failed to capture track description");
				continue;
			};

			let (_, [description]) = description_capture.extract();

			let metadata = TrackMetadata {
				title: title.to_string(),
				description: description.to_string(),
			};

			tracks.push(ProfileTrackListing { pointer, metadata });
		}

		if tracks.is_empty() {
			return Err(format!(
				"No valid track listings found in profile page HTML"
			));
		}

		Ok(Self {
			slug: tracks.first().unwrap().pointer.profile_slug.clone(),
			tracks,
		})
	}

	pub async fn add_to_library(&self, context: &mut crate::Context) -> Result<(), String> {
		for track in &self.tracks {
			let track_pointer = &track.pointer;
			debug!(
				"Adding track {} to profile {}",
				track_pointer.track_slug, self.slug
			);

			let Some((metadata, sound_pointer)) = track_pointer.fetch_track_page().await else {
				return Err(format!(
					"Failed to fetch sound pointer for track {}",
					track_pointer.track_slug
				));
			};

			let audio_track = SoundgasmAudioTrack::new(track_pointer.clone(), metadata, sound_pointer);
			audio_track.add_to_library(context).await;
		}

		Ok(())
	}
}

pub struct ProfileTrackListing {
	pub(super) pointer: TrackPointer,
	pub(super) metadata: TrackMetadata,
}

lazy_static! {
	static ref TRACK_SECTION_RE: Regex =
		Regex::new("<div class=\"sound-details\">(.+?)</div>").unwrap();
	static ref TRACK_URL_AND_TITLE_RE: Regex = Regex::new("<a href=\"(.+?)\">(.+?)</a>").unwrap();
	static ref TRACK_DESCRIPTION_RE: Regex =
		Regex::new("<span class=\"soundDescription\">(.+?)</span>").unwrap();
}

#[cfg(test)]
mod tests {
	use super::Profile;

	#[test]
	fn test_parse_profile_from_html() {
		let profile_html =
			include_str!("../../../../test/fixtures/http/soundgasm/profiles/sgdl-test/index.html");

		// With subdomain
		let profile = Profile::from_html(profile_html).unwrap();
		assert_eq!(profile.slug, "sgdl-test");
		assert_eq!(profile.tracks.len(), 1);

		assert_eq!(profile.tracks[0].pointer.profile_slug, "sgdl-test");
		assert_eq!(
			profile.tracks[0].pointer.track_slug,
			"shopping-mall-half-open-Netherlands-207-AM-161001_0998"
		);
		assert_eq!(
			profile.tracks[0].metadata.title,
			"shopping mall half open Netherlands 207 AM 161001_0998"
		);
		assert_eq!(
			profile.tracks[0].metadata.description,
			"Test audio from https://freesound.org/people/klankbeeld/sounds/808487/"
		);
	}
}
