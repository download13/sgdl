// @generated automatically by Diesel CLI.

diesel::table! {
    tracks (profile_slug, track_slug) {
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
