use log::info;

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

			let track = SoundgasmAudioTrack::from_track_page(track_pointer, metadata, sound_pointer);

			track.add_to_library(context).await;
			info!("Added Soundgasm track to library: {}", media_string);

			let profile_pointer = ProfilePointer::from(track.pointer);

			let profile = profile_pointer.fetch_profile().await;

			if let Err(err) = profile.add_to_library(context).await {
				info!("Failed to add profile to library: {:?}", err);
			} else {
				info!("Profile added to library successfully");
			}
		}
		Some(PointerType::SoundgasmProfile(profile)) => {
			// Handle Soundgasm profile logic
			info!("Scanning Soundgasm profile: {:?}", profile);
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
