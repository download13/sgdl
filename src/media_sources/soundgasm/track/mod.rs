mod metadata;
mod pointer;
mod row;
mod sound_pointer;
mod stored_audio;

use log::debug;

pub use metadata::TrackMetadata;
pub use pointer::TrackPointer;
pub use row::SoundgasmAudioTrackRow;
pub use sound_pointer::TrackSoundPointer;
pub use stored_audio::SoundgasmTrackAudio;

use crate::media_types::{MediaBlobPointer, MediaPointer, MediaType};
use crate::{media_sources::ProviderType, media_types::MediaItem, Context};

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
		MediaType::AudioMp3
	}

	fn get_title(&self) -> String {
		self.metadata.title.clone()
	}

	fn get_description(&self) -> String {
		self.metadata.description.clone()
	}

	fn get_author(&self) -> String {
		self.pointer.profile_slug.clone()
	}

	fn get_pointer(&self) -> impl MediaPointer {
		self.pointer.clone()
	}

	fn get_blob_pointer(&self) -> impl MediaBlobPointer {
		self.sound_pointer.clone()
	}

	async fn search(context: &mut Context, query: &str) -> Vec<SoundgasmAudioTrack> {
		use crate::schema::soundgasm_tracks::dsl::*;
		use diesel::dsl::sql;
		use diesel::prelude::*;
		use diesel::sql_types::Bool;

		// Split search terms into keywords and search for them in the title and description columns
		let conditionals = query
			.split_whitespace()
			.filter_map(|term| {
				if term.is_empty() {
					None
				} else {
					Some(format!(
						"title LIKE %{}% OR description LIKE %{}%",
						term, term
					))
				}
			})
			.collect::<Vec<_>>()
			.join(" OR ");

		let conditionals = sql::<Bool>(&conditionals);

		// Fixes?
		// Create a dedicated blocking thread that handles database requests from a queue and holds it's own connection handle

		// let task = tokio::task::spawn_blocking(move || {
		let rows = crate::schema::soundgasm_tracks::table
			.filter(conditionals)
			.select(SoundgasmAudioTrackRow::as_select())
			.load::<SoundgasmAudioTrackRow>(&mut context.conn)
			.unwrap_or_default();
		// });

		rows
			.iter()
			.filter_map(|row| SoundgasmAudioTrack::try_from(row).ok())
			.collect::<Vec<_>>()
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

impl TryFrom<&SoundgasmAudioTrackRow> for SoundgasmAudioTrack {
	type Error = String;

	fn try_from(value: &SoundgasmAudioTrackRow) -> Result<Self, Self::Error> {
		Ok(Self {
			metadata: TrackMetadata::from(value),
			pointer: TrackPointer::from(value),
			sound_pointer: TrackSoundPointer::try_from(value)?,
			stored_audio: SoundgasmTrackAudio::try_from(value).ok(),
		})
	}
}
