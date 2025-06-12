mod kemono;
mod soundgasm;

use crate::media_sources::soundgasm::{SoundgasmProfilePointer, SoundgasmTrackPointer};
use crate::media_types::MediaItem;

pub enum MediaSource {
	Soundgasm,
	Kemono,
}

pub enum PointerType {
	SoundgasmTrack(SoundgasmTrackPointer),
	SoundgasmProfile(SoundgasmProfilePointer),
	KemonoPost,
	KemonoProfile,
}

pub trait MediaPointer {
	fn get_source(&self) -> MediaSource;
	async fn fetch_metadata(&self) -> Vec<impl MediaItem>;
}

pub trait MediaItemPointer {
	fn get_source(&self) -> MediaSource;
	async fn fetch_metadata(&self) -> Vec<impl MediaItem>;
}

pub fn recognize_media_source_from_string(media_string: &str) -> Option<PointerType> {
	let track_pointer = SoundgasmTrackPointer::try_parse(media_string);
}
