mod metadata;
mod pointer;
mod row;
mod sound_pointer;
mod stored_audio;

use log::debug;

pub use metadata::TrackMetadata;
pub use pointer::{TrackPointer, TRACK_SLUG_PATTERN};
pub use row::SoundgasmAudioTrackRow;

use crate::Context;
use sound_pointer::TrackSoundPointer;
use stored_audio::SoundgasmTrackAudio;

#[derive(Debug, Clone)]
pub struct SoundgasmAudioTrack {
	pub pointer: TrackPointer,
	pub metadata: TrackMetadata,
	pub sound_pointer: TrackSoundPointer,
	pub stored_audio: Option<SoundgasmTrackAudio>,
}

impl SoundgasmAudioTrack {
	pub fn new(
		pointer: TrackPointer,
		metadata: TrackMetadata,
		sound_pointer: TrackSoundPointer,
	) -> Self {
		Self {
			pointer,
			metadata: metadata.clone(),
			sound_pointer: sound_pointer.clone(),
			stored_audio: None,
		}
	}
}

impl SoundgasmAudioTrack {
	pub async fn add_to_library(&self, context: &mut Context) {
		let row = SoundgasmAudioTrackRow::from(self.clone());

		let result = row.add_to_library(context).await;

		if let Some(updated_row) = result {
			debug!("Track metadata upserted successfully: {:?}", updated_row);
		} else {
			debug!("Failed to upsert track metadata");
		};
	}
}
