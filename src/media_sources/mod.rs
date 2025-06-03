use crate::media_types::MediaItem;

pub enum MediaSourceType {
	Soundgasm = 0,
	Kemono = 1,
}

pub(crate) async fn extract_media_items(media_string: &str) -> Vec<MediaItem> {
	let source = recognize_media_source_from_string(media_string);

	match source {
		Some(MediaSourceType::Soundgasm) => {
			let track = soundgasm::parse_track(media_string).await;
			let profile = soundgasm::parse_profile(media_string).await;

			if let Some(track) = track {
				vec![track]
			} else if let Some(profile) = profile {
				profile.get_tracks().await
			} else {
				vec![]
			}
		}
		Some(MediaSourceType::Kemono) => kemonoparty::extract_media_items(media_string).await,
		None => vec![],
	}
}

fn recognize_media_source_from_string(media_string: &str) -> Option<MediaSourceType> {
	if media_string.contains("soundgasm.net") {
		Some(MediaSourceType::Soundgasm)
	} else if media_string.contains("kemono.party")
		|| media_string.contains("kemono.su")
		|| media_string.contains("coomer.su")
	{
		Some(MediaSourceType::Kemono)
	} else {
		None
	}
}
