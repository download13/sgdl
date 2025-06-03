use diesel::prelude::*;
use schema::tracks::content_length;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use xxhash_rust::xxh3::Xxh3;

use crate::profile::ProfileTrackListing;
use crate::track::{TrackDetails, TrackId};

use self::schema::tracks::dsl as tracks_dsl;

mod audio_track;
#[path = "../schema.rs"]
pub mod schema;
mod soundgasm_track;

define_sql_function! {
	fn current_timestamp() -> Timestamp;
}

pub struct Store {
	conn: SqliteConnection,
	data_path: PathBuf,
}

impl Store {
	pub async fn new(data_path: &PathBuf) -> Result<Store, String> {
		println!("data_path {:?}", data_path);

		let conn = Self::establish_connection(&data_path);

		Result::Ok(Store {
			conn,
			data_path: data_path.clone(),
		})
	}

	fn establish_connection(data_path: &PathBuf) -> SqliteConnection {
		let audio_path = data_path.join("audio");
		if !audio_path.exists() {
			std::fs::create_dir_all(audio_path).unwrap();
		}

		let database_path = data_path.join("data/meta.sqlite3");
		let database_str = database_path.to_str().unwrap();

		SqliteConnection::establish(database_str).unwrap_or_else(|err| {
			println!("Error connecting to {err:?}");
			panic!("Error connecting to {}", database_str);
		})
	}

	fn get_audio_path(&self) -> PathBuf {
		let audio_path = self.data_path.join("audio");
		if !audio_path.exists() {
			std::fs::create_dir_all(&audio_path).unwrap();
		}

		audio_path
	}

	pub async fn search_tracks(&mut self, terms: String) -> Vec<Track> {
		use tracks_dsl::{description, profile_slug, title, track_slug};

		let formatted_terms = format!("%{}%", terms);

		let result = tracks_dsl::tracks
			.select(TrackRow::as_select())
			.filter(
				title
					.like(formatted_terms.clone())
					.or(description.like(formatted_terms.clone()))
					.or(track_slug.like(formatted_terms.clone()))
					.or(profile_slug.like(formatted_terms.clone())),
			)
			.load::<Track>(&mut self.conn);

		let Ok(tracks) = result else {
			println!("Error fetching tracks from database");
			println!("Error: {:?}", result.unwrap_err());
			return vec![];
		};

		tracks
	}

	pub async fn get_track(&mut self, profile_slug: String, track_slug: String) -> Option<Track> {
		use tracks_dsl::tracks;
		// Check if the track exists in the database
		let result = tracks
			.select(Track::as_select())
			.find((profile_slug.clone(), track_slug.clone()))
			.get_result(&mut self.conn);

		let Ok(track) = result else {
			println!("Error fetching track from database");
			println!("Error: {:?}", result.unwrap_err());
			return None;
		};

		if self.verify_audio_file(&track).await {
			return Some(track);
		}

		None
	}

	pub async fn upsert_track_listings(&mut self, track_listings: &Vec<ProfileTrackListing>) -> u64 {
		use diesel::insert_into;
		use schema::tracks::table;

		let tracks: Vec<Track> = track_listings
			.iter()
			.map(|listing| Track {
				id: TrackId {
					profile_slug: listing.profile_slug.clone(),
					track_slug: listing.track_slug.clone(),
				},
				details: TrackDetails {
					title: listing.title.clone(),
					description: listing.description.clone(),
					sound_id: None,  // Placeholder, not used in this context
					extension: None, // Default extension, can be changed later
				},
				audio: None,
			})
			.collect();

		let mut added_count: u64 = 0;
		for track in &tracks {
			let result = insert_into(table)
				.values(track)
				.returning(Track::as_returning())
				.get_result(&mut self.conn);

			let Ok(inserted_track) = result else {
				println!("Error inserting track into database");
				println!("Error: {:?}", result.unwrap_err());
				continue;
			};

			added_count += 1;
		}

		added_count
	}

	// TODO: Convert the print messages and failures to some kind of stream or progress reporting
	async fn verify_audio_file(&self, track: &Track) -> bool {
		let audio_path = self.get_audio_path();
		let track_path = self.get_track_path(&track);
		if !track_path.is_file() {
			return false;
		}

		let Ok(mut file) = File::open(&track_path).await else {
			return false;
		};

		let Ok(metadata) = file.metadata().await else {
			return false;
		};

		let content_hash = track.audio.content_hash;
		let content_length = track.audio.content_length as u64;

		if metadata.len() != content_length {
			println!("File size mismatch");
			println!("Expected: {:?}", track.audio.content_length);
			println!("Actual: {}", metadata.len());
			return false;
		}

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
		let hash_hex = format!("{:x}", hash);
		if hash_hex != *content_hash {
			println!("File hash mismatch");
			println!("Expected: {:?}", content_hash);
			println!("Actual: {}", hash_hex);
			return false;
		}
		println!("File verified successfully");
		println!("File path: {}", track_path.display());
		println!("File hash: {}", hash_hex);
		println!("File size: {}", metadata.len());

		true
	}

	pub async fn stream_response_to_track(
		&self,
		track: Track,
		mut res: reqwest::Response,
	) -> Option<Track> {
		let id = track.id.clone();

		let file_path = self.get_track_path(&track);
		if file_path.exists() {
			println!("File already exists: {}", file_path.display());
			return None;
		}

		let mut file = match File::create(file_path).await {
			Ok(file) => file,
			Err(err) => {
				println!("Error creating file: {}", err);
				return None;
			}
		};

		let mut hasher = Xxh3::new();

		loop {
			match res.chunk().await {
				Err(err) => {
					println!("Error reading response: {:?}", err);
					return None;
				}
				Ok(Some(data)) => {
					hasher.update(&data);
					file.write(data.as_ref()).await.unwrap();
				}
				Ok(None) => {
					break;
				}
			}
		}

		let hash = hasher.digest();
		let hash_hex = format!("{:x}", hash);
		let content_length = file.metadata().await.unwrap().len();

		// TODO: save track to database

		Some(track)
	}

	fn get_track_path(&self, track: &Track) -> PathBuf {
		self
			.get_audio_path()
			.join(&track.id.profile_slug)
			.join(&track.id.track_slug)
			.with_extension(&track.details.extension)
	}
}
