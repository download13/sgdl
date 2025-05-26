use crate::{
	common::{fetch_text, stream_bytes},
	store::{Store, Track},
	track::{TrackDetails, TrackId},
	Context,
};

pub async fn add_track(context: Context, track_url: String) {
	let track_id = match TrackId::new(&track_url) {
		Some(track_id) => track_id,
		None => {
			println!("Invalid track url: {}", &track_url);
			return;
		}
	};

	let Some(track_page_html) = track_id.get_track_page_html().await else {
		return;
	};

	let track_details = match TrackDetails::parse_track_details(&track_page_html) {
		Some(track_details) => track_details,
		None => {
			println!("Error parsing track page");
			return;
		}
	};

	try_download_track(track_id, track_details, context.store).await;
}

async fn try_download_track(track_id: TrackId, track_details: TrackDetails, store: Store) -> bool {
	let res = stream_bytes(track_details.get_audio_url()).await.unwrap();

	let track = Track::from((track_id, track_details));

	let track = store.stream_response_to_track(track, res).await;

	track.is_some()
}
