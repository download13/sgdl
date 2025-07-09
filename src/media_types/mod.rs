use crate::media_sources::ProviderType;

pub enum MediaType {
	Audio = 0,
	Video = 1,
	Image = 2,
	Text = 3,
	Pdf = 4,
}

// Things you can do with each media source:
// Scan SG Profile -> Add profile to library
// Scan KemonoProfile -> Add profile to library, add all posts to library
// Add SG Track -> Add track to library
// Add KemonoPost -> Add post to library, add all media items to library

// Things you can get from a pointer:
// SG Profile -> Vec<impl MediaPointer>
// SG Track -> impl AudioTrack
// KemonoProfile -> Vec<KemonoPage>
// KemonoPage -> Vec<KemonoPost>
// KemonoPost -> Vec<MediaItem>

pub trait TrackListing {
	fn get_provider(&self) -> ProviderType;
	fn get_source(&self) -> ProviderType;
	fn get_type() -> MediaType;

	async fn try_download(&self) -> bool;
	async fn verify_metadata(&self) -> bool;
	async fn verify_blob(&self) -> bool;
}

pub trait MediaItem {
	fn get_source(&self) -> ProviderType;
	fn get_type() -> MediaType;

	async fn try_download(&self) -> bool;
	async fn verify_metadata(&self) -> bool;
	async fn verify_blob(&self) -> bool;
}
