pub mod download_manager;
mod media_blob;

use diesel::prelude::*;
use std::path::{Path, PathBuf};
use tokio::fs::create_dir_all;

pub use media_blob::MediaBlob;

define_sql_function! {
	fn current_timestamp() -> Timestamp;
}

#[derive(Clone, Debug)]
pub struct FileStore {
	pub data_path: PathBuf,
}

impl FileStore {
	pub async fn new<P: AsRef<Path>>(data_path: P) -> Self {
		let this = Self {
			data_path: data_path.as_ref().to_path_buf(),
		};

		this.ensure_data_path().await;

		this
	}

	async fn ensure_data_path(&self) {
		if !self.data_path.exists() {
			let Err(err) = create_dir_all(&self.data_path).await else {
				return;
			};

			log::error!("Failed to create data directory: {}", err);
		}
	}

	// async fn get_namespace_path(&self, namespace: &str) -> PathBuf {
	// 	let namespace_path = self.data_path.join(namespace);
	// 	if !namespace_path.exists() {
	// 		match create_dir_all(&namespace_path).await {
	// 			Err(err) => {
	// 				log::error!("Failed to create namespace directory: {}", err);
	// 				return PathBuf::new(); // Return an empty path if creation fails
	// 			}
	// 			_ => {}
	// 		};
	// 	}

	// 	namespace_path
	// }

	// TODO: Convert the print messages and failures to some kind of stream or progress reporting

	// pub async fn stream_response_to_track(
	// 	&self,
	// 	track: Track,
	// 	mut res: reqwest::Response,
	// ) -> Option<Track> {
	// 	let id = track.id.clone();

	// 	let file_path = self.get_track_path(&track);
	// 	if file_path.exists() {
	// 		println!("File already exists: {}", file_path.display());
	// 		return None;
	// 	}

	// 	let mut file = match File::create(file_path).await {
	// 		Ok(file) => file,
	// 		Err(err) => {
	// 			println!("Error creating file: {}", err);
	// 			return None;
	// 		}
	// 	};

	// 	let mut hasher = Xxh3::new();

	// 	loop {
	// 		match res.chunk().await {
	// 			Err(err) => {
	// 				println!("Error reading response: {:?}", err);
	// 				return None;
	// 			}
	// 			Ok(Some(data)) => {
	// 				hasher.update(&data);
	// 				file.write(data.as_ref()).await.unwrap();
	// 			}
	// 			Ok(None) => {
	// 				break;
	// 			}
	// 		}
	// 	}

	// 	let hash = hasher.digest();
	// 	let hash_hex = format!("{:x}", hash);
	// 	let content_length = file.metadata().await.unwrap().len();

	// 	// TODO: save track to database

	// 	Some(track)
	// }
}
