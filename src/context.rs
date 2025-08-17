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

	pub fn add_url(&mut self, url: String) {
		use crate::media_sources::{recognize_pointer_from_string, PointerType};

		log::debug!("Adding URL: {}", url);

		recognize_pointer_from_string(&url).map(|pointer| {
			match pointer {
				PointerType::SoundgasmTrack(track) => {
					log::debug!("Adding Soundgasm track: {:?}", track);
					SoundgasmAudioTrack::add(self, track);
				}
				PointerType::SoundgasmProfile(profile) => {
					log::debug!("Adding Soundgasm profile: {:?}", profile);
					SoundgasmAudioTrack::add_profile(self, profile);
				}
				PointerType::KemonoPost(post) => {
					log::debug!("Adding Kemono post: {:?}", post);
					// Handle Kemono post addition
				}
				PointerType::KemonoProfile(profile) => {
					log::debug!("Adding Kemono profile: {:?}", profile);
					// Handle Kemono profile addition
				}
			}
		});
	}
}
