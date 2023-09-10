use regex::Regex;
use std::path::PathBuf;

use crate::common::fetch_text;

#[derive(Debug)]
struct TrackUrlInfo {
	profile_slug: String,
	track_slug: String,
}

pub fn command(data_path: PathBuf, track_url: String) {
	println!("track {}", track_url);

	// const store = new Store({ dataPath: options.dataPath })

	let track_info = parse_track_url_info(track_url);
	println!("track_info {:#?}", track_info);

	// if (!trackInfo) {
	// 	return console.log('invalid track url')
	// }

	// await ensureTrackDownloaded(trackInfo, store)
}

fn parse_track_url_info(url: String) -> Option<TrackUrlInfo> {
	let track_url_re =
		Regex::new(r"https://soundgasm.net/u/([0-9a-zA-Z_-]+)/([0-9a-zA-Z_-]+)").unwrap();

	let matches = track_url_re.captures(url.as_str())?;
	let profile_slug = String::from(matches.get(1).unwrap().as_str());
	let track_slug = String::from(matches.get(2).unwrap().as_str());

	Some(TrackUrlInfo {
		profile_slug,
		track_slug,
	})
}

async fn get_track_download_url(track: TrackUrlInfo) {
	let track_page_html = fetch_text(format!(
		"https://soundgasm.net/u/${track.profileSlug}/${track.trackSlug}"
	))
	.await;

	let re = Regex::new("https://media.soundgasm.net/sounds/([0-9a-f]+).([0-9a-zA-Z]+)");

	// let m = re.captures(track_page_html);

	// if (!match) {
	// 	return null
	// }

	// const [, soundId, extension] = match

	// return {
	// 	url: `https://media.soundgasm.net/sounds/${soundId}.${extension}`,
	// 	extension,
	// }
}
