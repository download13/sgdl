// @generated automatically by Diesel CLI.

diesel::table! {
    downloaded_segments (rowid) {
        rowid -> Integer,
        download_id -> Integer,
        start_index -> Integer,
        end_index -> Integer,
    }
}

diesel::table! {
    file_downloads (id) {
        id -> Integer,
        url -> Text,
        file_path -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    soundgasm_tracks (profile_slug, track_slug) {
        profile_slug -> Text,
        track_slug -> Text,
        title -> Text,
        description -> Text,
        sound_id -> Nullable<Text>,
        file_extension -> Nullable<Text>,
        content_hash -> Nullable<Text>,
        content_length -> Nullable<BigInt>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(downloaded_segments -> file_downloads (download_id));

diesel::allow_tables_to_appear_in_same_query!(
    downloaded_segments,
    file_downloads,
    soundgasm_tracks,
);
