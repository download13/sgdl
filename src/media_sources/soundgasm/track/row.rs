use std::fmt::Debug;

use diesel::prelude::*;
use log::debug;

// use crate::generate_update_type;
use super::SoundgasmAudioTrack;
use crate::schema;
use crate::Context;

#[derive(Debug, Clone, Selectable, Insertable, Queryable, QueryableByName)]
#[diesel(table_name = crate::schema::soundgasm_tracks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SoundgasmAudioTrackRow {
	pub profile_slug: String,
	pub track_slug: String,
	pub title: String,
	pub description: String,
	pub sound_id: Option<String>,
	pub file_extension: Option<String>,
	pub content_hash: Option<String>,
	pub content_length: Option<i64>,
	pub created_at: chrono::NaiveDateTime,
	pub updated_at: chrono::NaiveDateTime,
	pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl SoundgasmAudioTrackRow {
	pub async fn add_to_library(&self, context: &mut Context) -> Option<Self> {
		use schema::soundgasm_tracks::{profile_slug, table, track_slug};

		// TODO: Only add columns that are not null

		let updates = SoundgasmAudioTrackRowUpdate {
			profile_slug: None,
			track_slug: None,
			title: if !self.title.is_empty() {
				Some(self.title.clone())
			} else {
				None
			},
			description: if !self.description.is_empty() {
				Some(self.description.clone())
			} else {
				None
			},
			sound_id: self.sound_id.as_ref().map(|s| Some(s.clone())),
			file_extension: self.file_extension.as_ref().map(|s| Some(s.clone())),
			content_hash: self.content_hash.as_ref().map(|s| Some(s.clone())),
			content_length: self.content_length.map(Some),
			created_at: None,
			updated_at: Some(chrono::Utc::now().naive_utc()),
			deleted_at: None,
		};

		let result = diesel::insert_into(table)
			.values(self)
			.on_conflict((profile_slug, track_slug))
			.do_update()
			.set(updates)
			.returning(Self::as_returning())
			.get_result(&mut context.conn);

		match result {
			Ok(updated_row) => {
				debug!("Track metadata upserted successfully: {:?}", updated_row);
				Some(updated_row)
			}
			Err(err) => {
				debug!("Failed to upsert track metadata: {:?}", err);
				None
			}
		}
	}
}

impl From<SoundgasmAudioTrack> for SoundgasmAudioTrackRow {
	fn from(track: SoundgasmAudioTrack) -> Self {
		let (content_hash, content_length) = match track.stored_audio {
			Some(audio) => (Some(audio.content_hash), Some(audio.content_length)),
			None => (None, None),
		};

		Self {
			profile_slug: track.pointer.profile_slug,
			track_slug: track.pointer.track_slug,
			title: track.metadata.title,
			description: track.metadata.description,
			sound_id: Some(track.sound_pointer.sound_id),
			file_extension: Some(track.sound_pointer.file_extension.clone()),
			content_hash,
			content_length,
			created_at: chrono::Utc::now().naive_utc(),
			updated_at: chrono::Utc::now().naive_utc(),
			deleted_at: None,
		}
	}
}

#[derive(AsChangeset)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = schema::soundgasm_tracks)]
pub struct SoundgasmAudioTrackRowUpdate {
	profile_slug: Option<String>,
	track_slug: Option<String>,
	title: Option<String>,
	description: Option<String>,
	sound_id: Option<Option<String>>,
	file_extension: Option<Option<String>>,
	content_hash: Option<Option<String>>,
	content_length: Option<Option<i64>>,
	created_at: Option<chrono::NaiveDateTime>,
	updated_at: Option<chrono::NaiveDateTime>,
	deleted_at: Option<Option<chrono::NaiveDateTime>>,
}
