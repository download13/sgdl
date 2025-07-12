use log::{error, info};

use crate::media_sources::soundgasm::{ProfilePointer, SoundgasmAudioTrack};
use crate::media_sources::{recognize_pointer_from_string, PointerType};
use crate::Context;

// TODO: plan for how to handle different media sources
// star simple, check for sources, then add each to the library

pub async fn scan_command(media_string: String, context: &mut Context) {
	let source = recognize_pointer_from_string(&media_string);

	match source {
		None => {
			info!("Unrecognized media source for: {}", media_string);
		}
		Some(PointerType::SoundgasmTrack(track_pointer)) => {
			info!("Scanning Soundgasm track: {}", media_string);
			// Add track to library and mark for download
			let Some((metadata, sound_pointer)) = track_pointer.fetch_track_page().await else {
				info!("Failed to fetch soundgasm track page: {}", media_string);
				return;
			};

			let track = SoundgasmAudioTrack::new(track_pointer, metadata, sound_pointer);

			track.add_to_library(context).await;
			info!("Added Soundgasm track to library: {}", media_string);

			let profile_pointer = ProfilePointer::from(track.pointer);

			let scan_result = profile_pointer.scan(context).await;

			match scan_result {
				Ok(message) => info!("{}", message),
				Err(err) => info!("Error scanning profile: {}", err),
			};
		}
		Some(PointerType::SoundgasmProfile(pointer)) => {
			info!("Scanning Soundgasm profile: {}", pointer.slug);

			let scan_result = pointer.scan(context).await;

			match scan_result {
				Ok(msg) => info!("{}", msg),
				Err(err) => error!("Failed to add profile to library: {}", err),
			};
		}
		Some(PointerType::KemonoPost(post)) => {
			// Handle Kemono specific logic
			info!("Scanning Kemono media: {:?}", post);
		}
		Some(PointerType::KemonoProfile(profile)) => {
			// Handle Kemono profile logic
			info!("Scanning Kemono profile: {:?}", profile);
		}
	}
}
