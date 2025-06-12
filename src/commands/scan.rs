use crate::media_sources::{recognize_media_source_from_string, MediaSource, PointerType};
use crate::Context;

// TODO: plan for how to handle different media sources
// star simple, check for sources, then add each to the library

pub async fn scan_command(media_string: String, context: &mut Context) {
	let source = recognize_media_source_from_string(&media_string);

	match source {
		None => {
			println!("Unrecognized media source for: {}", media_string);
			return;
		}
		Some(PointerType::SoundgasmTrack()) => {
			// Handle Soundgasm specific logic
			println!("Scanning Soundgasm media: {}", media_string);
		}
		Some(MediaSource::Kemono) => {
			// Handle Kemono specific logic
			println!("Scanning Kemono media: {}", media_string);
		}
	}
	let source = source.unwrap();
}
