use diesel::SqliteConnection;

use crate::{
	config::Config,
	file_store::FileStore,
	media_sources::soundgasm::SoundgasmAudioTrack,
	media_types::{MediaItem, MediaType},
};

pub struct Context {
	pub config: Config,
	pub conn: SqliteConnection,
	pub file_store: FileStore,
}

impl Context {
	pub async fn search(
		&mut self,
		query: &str,
		filter_media_type: Option<MediaType>,
		// filter_provider_type: Option<ProviderType>,
	) -> Vec<impl MediaItem> {
		match filter_media_type {
			Some(MediaType::AudioMp3) => SoundgasmAudioTrack::search(self, query).await,
			_ => SoundgasmAudioTrack::search(self, query).await,
		}
	}
}
