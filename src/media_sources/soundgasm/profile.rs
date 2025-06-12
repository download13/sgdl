use crate::{common::PROFILE_PATTERN, track::TrackId};
use lazy_static::lazy_static;
use regex::Regex;

pub struct SoundgasmProfilePointer {
	slug: String,
}

lazy_static! {
	static ref PROFILE_URL_RE: Regex =
		Regex::new(format!("//(?:www.)?soundgasm.net/u/([{}]+)", PROFILE_PATTERN).as_str()).unwrap();
}

impl SoundgasmProfilePointer {
	pub fn parse(profile_id_or_url: &String) -> Option<SoundgasmProfilePointer> {
		let profile_slug = Self::parse_profile_slug(profile_id_or_url)?;
		if profile_slug.is_empty() {
			return None;
		}

		Some(AudioProfileId { profile_slug })
	}

	pub fn parse_profile_slug(profile_id_or_url: &String) -> Option<String> {
		let profile_slug = PROFILE_URL_RE.captures(profile_id_or_url)?;

		let profile_slug = profile_slug.unwrap().get(1).unwrap().as_str();
		profile_slug.to_string()
	}
}

struct AudioProfile {
	slug: String,
	tracks: Vec<ProfileTrackListing>,
}

pub struct ProfileTrackListing {
	pub profile_slug: String,
	pub track_slug: String,
	pub title: String,
	pub description: String,
}

lazy_static! {
	static ref TRACK_SECTION_RE: Regex =
		Regex::new("<div class=\"sound-details\">(.+?)</div>").unwrap();
	static ref TRACK_URL_RE: Regex = Regex::new("<a href=\"(.+?)\"").unwrap();
	static ref TRACK_TITLE_RE: Regex = Regex::new("<a href=\"(?:.+?)\">(.+?)</a>").unwrap();
	static ref TRACK_DESCRIPTION_RE: Regex =
		Regex::new("<span class=\"soundDescription\">(.+?)</span>").unwrap();
}

impl ProfileTrackListing {
	// TODO: Copy code from track.rs, or pull out into a parsing module
	pub fn parse_from_html(html: &str) -> Vec<ProfileTrackListing> {
		let mut track_listings = Vec::new();

		for section in TRACK_SECTION_RE.captures_iter(html) {
			let section_html = section.get(1).unwrap().as_str();

			let track_url = TRACK_URL_RE.captures(section_html).unwrap();
			let track_url = track_url.get(1).unwrap().as_str();

			let title = TRACK_TITLE_RE.captures(section_html).unwrap();
			let title = title.get(1).unwrap().as_str();

			let description = TRACK_DESCRIPTION_RE.captures(section_html).unwrap();
			let description = description.get(1).unwrap().as_str();

			let TrackId {
				profile_slug,
				track_slug,
				..
			} = TrackId::new(&track_url.to_string()).unwrap();

			track_listings.push(ProfileTrackListing {
				profile_slug,
				track_slug,
				title: title.to_string(),
				description: description.to_string(),
			});
		}

		track_listings
	}
}

#[cfg(test)]
mod tests {
	use crate::{config::Config, store::Store, Context};

	use super::*;

	#[test]
	fn test_parse_profile_slug() {
		// let slug = ProfileId::parse_profile_slug(&"//www.soundgasm.net/u/sgdl-test".to_string());
		// assert_eq!(slug, "sgdl-test");

		// // With trailing slash
		// let slug = ProfileId::parse_profile_slug(&"//www.soundgasm.net/u/sgdl-test/".to_string());
		// assert_eq!(slug, "sgdl-test");

		// // Track url
		// let slug = ProfileId::parse_profile_slug(
		// 	&"//www.soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998"
		// 		.to_string(),
		// );
		// assert_eq!(slug, "sgdl-test");

		// // Mild fuzzing
		// let slug =
		// 	ProfileId::parse_profile_slug(&"//www.soundgasm.net/u/!@#$$^&*()_+/!@#$^&*()_+".to_string());
		// assert_eq!(slug, "!@#$$^&*()_+");
	}

	#[tokio::test]
	async fn test_get_profile() {
		// let context = Context {
		// 	config: Config::new(),
		// 	store: Store::new(&"test.db".into()).await.unwrap(),
		// };

		// let track_info = get_profile(context, "sgdl-test".into()).await;
		// assert!(track_info.is_none());

		// // Close, but wrong domain
		// let track_info = TrackId::new(&"//soundgasm.com/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998".into());
		// assert!(track_info.is_none());

		// // Wrong subdomain
		// let track_info = TrackId::new(&"//dfs.soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998".into());
		// assert!(track_info.is_none());
	}
}
