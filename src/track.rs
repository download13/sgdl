use crate::common::fetch_text;
use crate::store::Store;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct TrackLocation {
	original_url: String,
	pub profile_slug: String,
	pub track_slug: String,
}

#[derive(Debug)]
pub struct TrackAudioContent {
	content_hash: String,
	content_length: u64,
}

#[derive(Debug)]
pub struct TrackAudioRemote {
	sound_id: String,
	extension: String,
}

pub fn command(store: Store, track_url: String) {
	println!("track {}", track_url);

	let track_info = parse_track_url_info(track_url);
	println!("track_info {:#?}", track_info);

	// if (!trackInfo) {
	// 	return console.log('invalid track url')
	// }

	// await ensureTrackDownloaded(trackInfo, store)
}

lazy_static! {
	static ref TRACK_URL_RE: Regex =
		Regex::new("//(?:www.)?soundgasm.net/u/([0-9a-zA-Z_-]+)/([0-9a-zA-Z_-]+)").unwrap();
}

fn parse_track_url_info(url: String) -> Option<Box<TrackLocation>> {
	let matches = TRACK_URL_RE.captures(url.as_str())?;

	let profile_slug = matches.get(1).unwrap().as_str().into();
	let track_slug = matches.get(2).unwrap().as_str().into();

	Some(Box::new(TrackLocation {
		original_url: url,
		profile_slug,
		track_slug,
	}))
}

lazy_static! {
	static ref TRACK_DOWNLOAD_RE: Regex =
		Regex::new("//media.soundgasm.net/sounds/([0-9a-f]+).([0-9a-zA-Z]+)").unwrap();
}

async fn get_track_download_url(track: TrackLocation) -> Option<TrackAudioRemote> {
	let track_page_url = format!(
		"//soundgasm.net/u/{}/{}",
		track.profile_slug, track.track_slug
	);
	let track_page_html = fetch_text(track_page_url).await?;

	let matches = TRACK_DOWNLOAD_RE.captures(track_page_html.as_str())?;

	let sound_id = String::from(matches.get(1).unwrap().as_str());
	let extension = String::from(matches.get(2).unwrap().as_str());

	Some(TrackAudioRemote {
		sound_id,
		extension,
	})
}

async fn ensure_track_downloaded(track: Track, store: Store) -> Result<(), reqwest::Error> {
	if store.has_track(track).await {
		return;
	}

	let download_info = get_track_download_url(track).await;

	if let Some(TrackRemoteMedia {
		sound_id,
		extension,
	}) = download_info
	{
		let sound_url = format!(
			"https://media.soundgasm.net/sounds/{}.{}",
			sound_id, extension
		);

		let res = reqwest::get(sound_url).await?;

		store.stream_response_to_track(res, track, extension)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::store::Store;
	use std::path::PathBuf;

	#[test]
	fn test_parse_track_url_info() {
		let track_info =
			parse_track_url_info("//www.soundgasm.net/u/Profess4orCal_/hi-everyone_2".into()).unwrap();
		assert_eq!(track_info.profile_slug, "Profess4orCal_");
		assert_eq!(track_info.track_slug, "hi-everyone_2");

		let track_info =
			parse_track_url_info("//www.soundgasm.net/u/!@#$$^&*()_+/!@#$^&*()_+".into()).unwrap();
		assert_eq!(track_info.profile_slug, "!@#$$^&*()_+");
		assert_eq!(track_info.track_slug, "!@#$^&*()_+");

		let track_info =
			parse_track_url_info("//soundgasm.net/u/Profess4orCal_/hi-everyone_2".into()).unwrap();
		assert_eq!(track_info.profile_slug, "Profess4orCal_");
		assert_eq!(track_info.track_slug, "hi-everyone_2");

		let track_info =
			parse_track_url_info("//soundgasm.com/u/Profess4orCal_/hi-everyone_2".into()).unwrap();
		assert!(track_info.is_none());

		let track_info =
			parse_track_url_info("//dfs.soundgasm.net/u/Profess4orCal_/hi-everyone_2".into()).unwrap();
		assert!(track_info.is_none());
	}

	#[test]
	fn test_parse_invalid_track_url() {
		let url = String::from("invalid_url");
		let track_info = parse_track_url_info(url);
		assert!(track_info.is_none());
	}
}
