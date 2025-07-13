mod metadata;
mod pointer;
mod row;
mod sound_pointer;
mod stored_audio;

use log::debug;

pub use metadata::TrackMetadata;
pub use pointer::{TrackPointer, TRACK_SLUG_PATTERN};
pub use row::SoundgasmAudioTrackRow;

use crate::media_types::{MediaPointer, MediaType};
use crate::{media_sources::ProviderType, media_types::MediaItem, Context};
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

impl MediaItem for SoundgasmAudioTrack {
	fn get_source(&self) -> ProviderType {
		ProviderType::Soundgasm
	}

	fn get_type(&self) -> MediaType {
		MediaType::Audio
	}

	fn get_pointer(&self) -> impl MediaPointer {
		self.pointer.clone()
	}

	async fn try_download(&self) -> bool {
		if let Some(stored_audio) = &self.stored_audio {
			if stored_audio.verify_blob().await {
				debug!(
					"Soundgasm track already downloaded: {}",
					self.pointer.to_url()
				);
				return true;
			} else {
				debug!(
					"Stored audio blob verification failed for: {}",
					self.pointer.to_url()
				);
			}
		}

		let result = audio.download().await;

		if result.is_ok() {
			self.stored_audio = Some(audio);
			true
		} else {
			debug!(
				"Failed to download Soundgasm track: {}",
				self.pointer.to_url()
			);
			false
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

	pub async fn search(context: &Context, query: &str) -> Vec<SoundgasmAudioTrack> {
		SoundgasmAudioTrackRow::search(context, query)
			.await
			.into_iter()
			.map(|row| SoundgasmAudioTrack::from(row))
			.collect()
	}
}
