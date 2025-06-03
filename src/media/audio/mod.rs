pub mod profile;

pub trait AudioItem: MediaItem {
	fn get_audio_source(&self) -> MediaSource;
	fn get_audio_type(&self) -> MediaType;
	fn get_audio_path(&self) -> String;
	async fn try_download_audio(&self) -> bool;
	async fn verify_audio_metadata(&self) -> bool;
	async fn verify_audio_blob(&self) -> bool;
}
