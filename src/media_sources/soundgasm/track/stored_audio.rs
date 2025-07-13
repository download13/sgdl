use crate::media_sources::soundgasm::SoundgasmAudioTrackRow;

#[derive(Debug, Clone)]
pub struct SoundgasmTrackAudio {
	pub content_hash: Option<String>,
	pub content_length: Option<i64>,
}

impl SoundgasmTrackAudio {
	fn from(track: SoundgasmAudioTrackRow) -> Self {
		Self {
			content_hash: track.content_hash,
			content_length: track.content_length,
		}
	}
}
