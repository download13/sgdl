use chrono::NaiveDateTime;
use diesel::deserialize::FromSqlRow;
use diesel::prelude::*;

use crate::track::TrackId;

#[path = "../../schema.rs"]
pub mod schema;

#[derive(Debug, Clone, Selectable, Insertable)]
#[diesel(table_name = schema::tracks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SoundgasmTrack {
	#[diesel(embed)]
	pub id: TrackId,
	#[diesel(embed)]
	pub details: TrackDetails,
	#[diesel(embed)]
	pub audio: TrackAudio,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::tracks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SoundgasmTrackRow {
	#[diesel(embed)]
	track: SoundgasmTrack,
	created_at: NaiveDateTime,
	updated_at: NaiveDateTime,
	deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Selectable)]
#[diesel(table_name = schema::tracks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct SoundgasmTrackAudio {
	pub content_hash: Option<String>,
	pub content_length: Option<i64>,
}

type TrackAudioRow = (diesel::sql_types::Text, diesel::sql_types::Integer);

impl FromSqlRow<TrackAudioRow, diesel::sqlite::Sqlite> for SoundgasmTrackAudio
where
	DB: diesel::backend::Backend,
{
	fn build_from_row<'a>(
		row: &impl diesel::row::Row<'a, diesel::sqlite::Sqlite>,
	) -> diesel::deserialize::Result<Self> {
		println!("blob: {:?}", row.get(0));
		println!("text: {:?}", row.get_value(0));

		panic!("test");

		Ok(SoundgasmTrackAudio {
			content_hash: "".to_string(),
			content_length: "".to_string(),
		})
	}
}
