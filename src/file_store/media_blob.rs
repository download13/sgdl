use std::path::PathBuf;

use tokio::{fs::File, io::AsyncReadExt};
use xxhash_rust::xxh3::Xxh3;

pub trait MediaBlob {
	fn get_path(&self) -> PathBuf;

	async fn get_content_hash(&self) -> Option<String> {
		let file_path = self.get_path();
		if !file_path.is_file() {
			return None;
		}

		let mut file = File::open(&file_path).await.ok()?;

		let mut buffer = vec![0; 8192];
		let mut hasher = Xxh3::new();

		loop {
			let Ok(bytes_read) = file.read(&mut buffer).await else {
				break;
			};

			if bytes_read == 0 {
				break;
			}

			hasher.update(&buffer[..bytes_read]);
			// TODO: Report progress with indicitif
		}

		let hash = hasher.digest();
		Some(format!("{:x}", hash))
	}

	async fn get_content_length(&self) -> Option<i64> {
		let path = self.get_path();
		tokio::fs::metadata(path)
			.await
			.ok()
			.map(|metadata| metadata.len() as i64)
	}
}
