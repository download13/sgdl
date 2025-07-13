use crate::{file_store::MediaBlob, media_sources::ProviderType};

pub enum MediaType {
	Audio = 0,
	Video = 1,
	Image = 2,
	Text = 3,
	Pdf = 4,
}

// Things you can do with each media source:
// Add SG Track -> Add track to library
// Scan KemonoProfile -> Add profile to library, add all posts to library
// Add KemonoPost -> Add post to library, add all media items to library

// Things you can get from a pointer:
// SG Profile -> Vec<impl MediaPointer>
// SG Track -> impl AudioTrack
// KemonoProfile -> Vec<KemonoPage>
// KemonoPage -> Vec<KemonoPost>
// KemonoPost -> Vec<MediaItem>

pub trait MediaPointer {
	async fn fetch_metadata(&self) -> Vec<impl MediaMetadata>;
	async fn fetch_blob_pointer(&self) -> Option<impl MediaBlob>;
}

pub trait MediaMetadata {
	async fn get_title(&self) -> String;
	async fn get_description(&self) -> String;
}

pub trait MediaItem {
	fn get_source(&self) -> ProviderType;
	fn get_type(&self) -> MediaType;

	fn get_pointer(&self) -> impl MediaPointer;

	async fn try_download(&self) -> bool;
}
