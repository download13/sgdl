use std::{collections::HashMap, io::SeekFrom, path::Path};

use futures_util::StreamExt;
use http_content_range::ContentRange;
use log::error;
use reqwest::{Response, Url};
use tokio::{
	fs::File,
	io::{AsyncSeekExt, AsyncWriteExt},
	sync::mpsc::{self, Receiver, Sender},
	task::JoinHandle,
};

use crate::media_types::{MediaBlobPointer, MediaItem};

pub struct DownloadManager {
	progress_tx: Sender<(Url, DownloadProgress)>,
	progress_rx: Receiver<(Url, DownloadProgress)>,
	downloads: HashMap<Url, JoinHandle<()>>,
	progress: HashMap<Url, DownloadProgress>,
}

impl Default for DownloadManager {
	fn default() -> Self {
		Self::new(32)
	}
}

impl DownloadManager {
	pub fn new(initial_concurrency: usize) -> DownloadManager {
		let (progress_tx, progress_rx) = mpsc::channel::<(Url, DownloadProgress)>(32);

		DownloadManager {
			progress_tx,
			progress_rx,
			downloads: HashMap::with_capacity(initial_concurrency),
			progress: HashMap::with_capacity(initial_concurrency),
		}
	}

	pub fn get_progress(&self) -> &HashMap<Url, DownloadProgress> {
		&self.progress
	}

	pub async fn start_download(
		&mut self,
		item: impl MediaItem,
		tx: Sender<(Url, DownloadProgress)>,
	) {
		let url = item.get_blob_pointer().get_download_url();
		// Ignore the request if we're already downloading the file
		if self.progress.contains_key(&url) {
			return;
		}

		let mut file = match File::open(item.get_blob_pointer().get_path()).await {
			Ok(file) => file,
			Err(err) => {
				error!("Unable to open file for download: {}", err);
				return;
			}
		};

		let handle = tokio::spawn(Self::run_download(
			user_agent,
			&url,
			file,
			self.progress_tx.clone(),
		));

		self.progress.insert(
			url.clone(),
			DownloadProgress {
				bytes_downloaded: 0,
				total_size: None,
			},
		);
	}

	pub async fn abort_download(&mut self) {}

	async fn run_download(
		user_agent: String,
		url: &Url,
		file: &mut File,
		tx: mpsc::Sender<(Url, DownloadProgress)>,
	) {
		let client = reqwest::Client::builder()
			.user_agent(user_agent)
			.build()
			.unwrap();

		let response = match client.get(url.clone()).send().await {
			Ok(response) => response,
			Err(err) => {
				// TODO: send back error progress
				return;
			}
		};

		Self::seek_to_content_range(&response, file);

		let mut stream = response.bytes_stream();

		// TODO: Track downloaded chunks in the database and calculate the progress based on those

		while let Some(chunk) = stream.next().await {
			let mut bytes = match chunk {
				Err(err) => {
					// TODO: Report error back
					return;
				}
				Ok(bytes) => bytes,
			};

			file.write_all(&mut bytes).await;

			if let Err(err) = tx
				.send((
					url.clone(),
					DownloadProgress {
						bytes_downloaded: bytes.len() as u64,
						total_size: file.stream_position().await.ok(),
					},
				))
				.await
			{
				error!("{:#?}", err)
			};
		}
	}

	fn seek_to_content_range(response: &Response, file: &mut File) -> Option<()> {
		let range_header = response.headers().get("Content-Range")?;

		let range = ContentRange::parse_bytes(&range_header.as_bytes())?;

		match range {
			ContentRange::Bytes(range) => {
				file.seek(SeekFrom::Start(range.first_byte));
			}
			ContentRange::UnboundBytes(range) => {
				file.seek(SeekFrom::Start(range.first_byte));
			}
			ContentRange::Unsatisfied(_range) => {
				return None;
			}
		};

		Some(())
	}
}

pub struct DownloadProgress {
	bytes_downloaded: u64,
	total_size: Option<u64>,
}
