use std::path::PathBuf;

use crate::{file_store::MediaBlob, media_sources::ProviderType, Context};

#[derive(strum_macros::Display, strum_macros::AsRefStr)]
pub enum MediaType {
	AudioMp3 = 0,
	VideoMp4 = 100,
	VideoWebm = 101,
	ImageJpg = 200,
	ImagePng = 201,
	Text = 300,
	Pdf = 301,
}

impl MediaType {
	fn get_extension(&self) -> &str {
		match self {
			Self::AudioMp3 => "mp3",
			Self::VideoMp4 => "mp4",
			Self::VideoWebm => "webm",
			Self::ImageJpg => "jpg",
			Self::ImagePng => "png",
			Self::Text => "txt",
			Self::Pdf => "pdf",
		}
	}
}

// impl Display for MediaType {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
// 		match
// 		write!(f, "{}", r)
// 	}
// }

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
	async fn fetch_blob_pointer(&self) -> Option<impl MediaBlobPointer>;
}

pub trait MediaBlobPointer {
	fn get_path(&self) -> PathBuf;
	fn get_download_url(&self) -> String;
}

pub trait MediaMetadata {
	fn get_title(&self) -> String;
	fn get_description(&self) -> String;
}

pub trait MediaItem
where
	Self: std::marker::Sized,
{
	fn get_source(&self) -> ProviderType;
	fn get_type(&self) -> MediaType;
	fn get_title(&self) -> String;
	fn get_description(&self) -> String;
	fn get_author(&self) -> String;

	fn get_pointer(&self) -> impl MediaPointer;
	fn get_blob_pointer(&self) -> impl MediaBlobPointer;

	async fn search(context: &mut Context, query: &str) -> Vec<Self>;
}
