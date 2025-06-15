use log::info;

use crate::media_sources::{recognize_pointer_from_string, MediaSource, PointerType};
use crate::store::add_metadata_to_library;
use crate::Context;

// TODO: plan for how to handle different media sources
// star simple, check for sources, then add each to the library

pub async fn scan_command(media_string: String, context: &mut Context) {
	let source = recognize_pointer_from_string(&media_string);

	match source {
		None => {
			info!("Unrecognized media source for: {}", media_string);
			return;
		}
		Some(PointerType::SoundgasmTrack(track)) => {
			info!("Scanning Soundgasm track: {}", media_string);
			// Add track to library and mark for download
			let Some(metadata) = track.fetch_metadata().await else {
				info!(
					"Failed to fetch metadata for Soundgasm track: {}",
					media_string
				);
				return;
			};

			add_metadata_to_library(context, metadata).await;
			info!("Added Soundgasm track to library: {}", media_string);
		}
		Some(PointerType::SoundgasmProfile(profile)) => {
			// Handle Soundgasm profile logic
			info!("Scanning Soundgasm profile: {}", media_string);
		}
		Some(PointerType::KemonoPost(post)) => {
			// Handle Kemono specific logic
			info!("Scanning Kemono media: {}", media_string);
		}
		Some(PointerType::KemonoProfile(profile)) => {
			// Handle Kemono profile logic
			info!("Scanning Kemono profile: {}", media_string);
		}
	}
}
