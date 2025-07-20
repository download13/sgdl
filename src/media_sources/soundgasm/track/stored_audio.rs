use std::path::PathBuf;

use crate::{
	file_store::MediaBlob,
	media_sources::soundgasm::{SoundgasmAudioTrackRow, TrackSoundPointer},
};

#[derive(Debug, Clone)]
pub struct SoundgasmTrackAudio {
	pub sound_pointer: TrackSoundPointer,
	pub content_hash: String,
	pub content_length: i64,
}

impl TryFrom<&SoundgasmAudioTrackRow> for SoundgasmTrackAudio {
	type Error = String;

	fn try_from(row: &SoundgasmAudioTrackRow) -> Result<Self, Self::Error> {
		let Some(content_length) = row.content_length.clone() else {
			return Err("content_length is None".to_string());
		};
		let Some(content_hash) = row.content_hash.clone() else {
			return Err("content_hash is None".to_string());
		};

		Ok(Self {
			sound_pointer: TrackSoundPointer::try_from(row)?,
			content_hash,
			content_length,
		})
	}
}

impl MediaBlob for SoundgasmTrackAudio {
	fn get_path(&self) -> PathBuf {
		PathBuf::from(format!(
			"data/soundgasm_audio/{}.{}",
			self.content_hash, self.sound_pointer.file_extension
		))
	}

	fn get_content_length(&self) -> i64 {
		self.content_length
	}

	fn get_content_hash(&self) -> String {
		self.content_hash.clone()
	}
}
