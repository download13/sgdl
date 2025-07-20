use std::{collections::HashMap, io::SeekFrom};

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

pub struct DownloadManager {
	progress_tx: Sender<(Url, DownloadProgress)>,
	progress_rx: Receiver<(Url, DownloadProgress)>,
	downloads: HashMap<Url, JoinHandle<()>>,
	progress: HashMap<Url, DownloadProgress>,
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

	async fn start_download(
		&mut self,
		download_request: DownloadRequest,
		tx: Sender<(Url, DownloadProgress)>,
	) {
		// Ignore the request if we're already downloading the file
		if self.progress.contains_key(&download_request.url) {
			return;
		}

		let url = download_request.url.clone();

		let handle = tokio::spawn(Self::run_download(
			download_request,
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

	async fn abort_download(&mut self) {}

	async fn run_download(
		download_request: DownloadRequest,
		tx: mpsc::Sender<(Url, DownloadProgress)>,
	) {
		let client = reqwest::Client::builder()
			.user_agent(download_request.user_agent)
			.build()
			.unwrap();

		let response = match client.get(download_request.url.clone()).send().await {
			Ok(response) => response,
			Err(err) => {
				// TODO: send back error progress
				return;
			}
		};

		let mut file = download_request.to_file;

		Self::seek_to_content_range(&response, &mut file);

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
					download_request.url.clone(),
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

struct DownloadRequest {
	user_agent: String,
	url: Url,
	to_file: File,
}

pub struct DownloadProgress {
	bytes_downloaded: u64,
	total_size: Option<u64>,
}
