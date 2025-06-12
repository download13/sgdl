use crate::media_types::MediaItem;
use core::panic;

pub struct AudioTrack {
	pub audio_url: String,
}

impl AudioTrack {
	fn get_audio_url(&self) -> String {
		self.audio_url.clone()
	}
}

impl MediaItem for AudioTrack {
	fn get_file_path(&self) -> String {
		self.get_audio_url()
	}

	async fn try_download(&self) -> bool {
		// let res = stream_bytes(self.get_audio_url()).await;
		panic!("Download not implemented for AudioTrack");
		false
	}

	async fn verify_metadata(&self) -> bool {
		// Placeholder for metadata verification logic
		panic!("Metadata verification not implemented for AudioTrack");
		false
	}

	async fn verify_blob(&self) -> bool {
		// Placeholder for blob verification logic
		panic!("Blob verification not implemented for AudioTrack");
		false
	}
}
