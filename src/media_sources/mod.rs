use self::soundgasm::TrackPointer;

mod kemono;
pub mod soundgasm;

// use crate::media_types::MediaItem;

pub enum ProviderType {
	Soundgasm,
	Kemono,
	Patreon,
}

pub enum PointerType {
	SoundgasmTrack(TrackPointer),
	SoundgasmProfile(soundgasm::ProfilePointer),
	KemonoPost(kemono::PostPointer),
	KemonoProfile(kemono::ProfilePointer),
}

// pub trait MediaPointer {
// 	fn get_source(&self) -> MediaSource;
// 	async fn fetch_metadata(&self) -> Vec<impl MediaItem>;
// }

// pub trait MediaItemPointer {
// 	fn get_source(&self) -> MediaSource;
// 	async fn fetch_metadata(&self) -> Vec<impl MediaItem>;
// }

pub fn recognize_pointer_from_string(media_string: &str) -> Option<PointerType> {
	if let Ok(sg_track_pointer) = soundgasm::TrackPointer::from_url(media_string) {
		return Some(PointerType::SoundgasmTrack(sg_track_pointer));
	}

	if let Some(sg_profile_pointer) = soundgasm::ProfilePointer::from_url(media_string) {
		return Some(PointerType::SoundgasmProfile(sg_profile_pointer));
	}

	if let Some(kemono_post_pointer) = kemono::PostPointer::from_url(media_string) {
		return Some(PointerType::KemonoPost(kemono_post_pointer));
	}

	if let Some(kemono_profile_pointer) = kemono::ProfilePointer::from_url(media_string) {
		return Some(PointerType::KemonoProfile(kemono_profile_pointer));
	}

	None
}
