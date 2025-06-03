use crate::{
	common::fetch_text,
	profile::{ProfileId, ProfileTrackListing},
	Context,
};

pub fn scan_command(media_string: String, context: &mut Context) {
	let source = recognize_media_source(&media_string);
	if source.is_none() {
		println!("Unrecognized media source for: {}", media_string);
		return;
	}

	match source.unwrap() {
		MediaSource::Soundgasm => {}
		MediaSource::Kemono => scan_kemono(media_string, context),
	}

	let track = parse_track(media_string);
	let profile = parse_profile(&media_string);

	let profile_url = format!("{}/{}", context.config.get_server_url(), profile_slug);
	let profile_page_html = match fetch_text(profile_url).await {
		Ok(html) => html,
		Err(err) => {
			println!("Error fetching profile page: {}", err);
			return;
		}
	};

	let track_listings = ProfileTrackListing::parse_from_html(&profile_page_html);
}

fn parse_track(media_string: String) -> TraclId {
	let track_id = TrackId::new(&media_string).await;
	let track_page_html = track_id
		.get_track_page()
		.await
		.expect("Failed to fetch track page");

	ProfileTrackListing::parse_from_html(&track_page_html)
}
