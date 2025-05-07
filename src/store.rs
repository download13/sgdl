use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

use crate::track::{TrackAudioContent, TrackAudioRemote, TrackLocation};

#[derive(Debug)]
pub struct Store {
	conn: Connection,
	audio_path: PathBuf,
}

#[derive(Debug)]
pub struct StoredTrack {
	loc: TrackLocation,
	audio: TrackAudioContent,
	remote: TrackAudioRemote,
}

#[derive(Debug)]
enum InitError {
	AudioDataPathIsNotDir,
	RusqliteError(rusqlite::Error),
}

impl Store {
	pub async fn new(data_path: PathBuf) -> Result<Store, InitError> {
		println!("data_path {:?}", data_path);

		let conn = match Self::db_setup(data_path) {
			Ok(conn) => conn,
			Err(err) => return Err(InitError::RusqliteError(err)),
		};

		Result::Ok(Store { conn, audio_path })
	}

	fn db_setup(data_path: PathBuf) -> Result<Connection, rusqlite::Error> {
		let conn = Connection::open(data_path.join("meta.db"))?;
		conn.execute(
			"CREATE TABLE IF NOT EXISTS tracks (
				id INTEGER PRIMARY KEY,
				profile_slug TEXT NOT NULL,
				track_slug TEXT NOT NULL,
				audio_path TEXT NOT NULL,
				remote_url TEXT NOT NULL
			)",
			params![],
		)?;
		conn.execute(
			"CREATE UNIQUE INDEX IF NOT EXISTS idx_tracks_profile_slug_track_slug ON tracks (profile_slug, track_slug)",
			params![],
		)?;
		conn.execute(
			"CREATE INDEX IF NOT EXISTS idx_tracks_profile_slug ON tracks (profile_slug)",
			params![],
		)?;

		Ok(conn)
	}

	pub async fn stream_response_to_track(
		&self,
		res: reqwest::Response,
		TrackLocation {
			profile_slug,
			track_slug,
			..
		}: TrackLocation,
		extension: String,
	) -> Option<TrackAudioContent> {
		let temp_file_name = "adsfa3434"; // TODO: generate random name, uuid
		let temp_file_path = self
			.audio_path
			.join(profile_slug)
			.join(track_slug)
			.with_file_name(temp_file_name);

		let file = match File::create(temp_file_path).await {
			Err(err) => return None,
			Ok(file) => file,
		};

		loop {
			match res.chunk().await {
				Err(err) => return None,
				Ok(None) => return None,
				Ok(Some(data)) => {
					file.write_all(data.as_ptr());
				}
			}
		}

		// TODO: download to temp file, hash contents, rename to hashed filename
	}

	/* async fn has_track(
		&self,
		TrackUrlInfo {
			profile_slug,
			track_slug,
		}: TrackUrlInfo,
	) -> bool {
		let profile_path = Path::new(&format!("{}/{}", self.data_path.display(), profile_slug));

		let md = metadata(profile_path).await;

		match md {
			Ok(r) => {
				println!("ok {:?}", r);

				let track_files = read_dir(profile_path).await;

				let track_slugs = track_files.map(|filename| basename(filename, filename.extension()));

				true
			}
			Err(err) => {
				println!("file access error {}", err);
				false
			}
		} */

	/* if (!trackSlugs.includes(trackSlug)) {
		return false
	}

	let info = await stat(`${profilePath}/${trackSlug}`)

	return info.size > 0
	} */
}
