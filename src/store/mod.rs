use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use xxhash_rust::xxh3::Xxh3;

use crate::profile::ProfileTrackListing;
use crate::track::{TrackDetails, TrackId};

use self::schema::tracks::dsl as tracks_dsl;

#[path = "../schema.rs"]
pub mod schema;

pub struct Store {
	conn: SqliteConnection,
	data_path: PathBuf,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::tracks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Track {
	pub fetch_url: String,
	pub profile_slug: String,
	pub track_slug: String,
	pub title: String,
	pub description: String,
	pub file_extension: String,
	pub content_hash: String,
	pub content_length: i64,
}

impl From<(TrackId, TrackDetails)> for Track {
	fn from((track_id, track_details): (TrackId, TrackDetails)) -> Self {
		Track {
			fetch_url: track_id.original_url,
			profile_slug: track_id.profile_slug,
			track_slug: track_id.track_slug,
			title: track_details.title,
			description: track_details.description,
			file_extension: track_details.extension,
			content_hash: String::new(),
			content_length: 0 as i64,
		}
	}
}

impl From<&ProfileTrackListing> for Track {
	fn from(track_listing: &ProfileTrackListing) -> Self {
		Track {
			fetch_url: String::new(),
			profile_slug: track_listing.profile_slug.clone(),
			track_slug: track_listing.track_slug.clone(),
			title: track_listing.title.clone(),
			description: track_listing.description.clone(),
			content_length: 0,
			content_hash: String::new(),
			file_extension: String::new(),
		}
	}
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
		dotenv().ok();

		let audio_path = data_path.join("audio");
		if !audio_path.exists() {
			std::fs::create_dir_all(audio_path).unwrap();
		}

		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

		SqliteConnection::establish(&database_url).unwrap_or_else(|err| {
			println!("Error connecting to {err:?}");
			panic!("Error connecting to {}", database_url);
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
			.select(Track::as_select())
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
			.map(|track| Track::from(track))
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

	async fn verify_audio_file(
		&self,
		Track {
			profile_slug,
			track_slug,
			file_extension,
			content_hash,
			content_length,
			..
		}: &Track,
	) -> bool {
		let audio_path = self.get_audio_path();
		let track_path = audio_path
			.join(profile_slug)
			.join(track_slug)
			.with_extension(file_extension);

		if !track_path.is_file() {
			return false;
		}

		let Ok(mut file) = File::open(&track_path).await else {
			return false;
		};

		let Ok(metadata) = file.metadata().await else {
			return false;
		};

		if metadata.len() != *content_length as u64 {
			println!("File size mismatch");
			println!("Expected: {}", content_length);
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
			println!("Expected: {}", content_hash);
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
		mut track: Track,
		mut res: reqwest::Response,
	) -> Option<Track> {
		let file_path = self
			.get_audio_path()
			.join(&track.profile_slug)
			.join(&track.track_slug)
			.with_extension(&track.file_extension);

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

		track.content_hash = hash_hex;
		track.content_length = content_length as i64;

		// TODO: save track to database

		Some(track)
	}
}
