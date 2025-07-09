// @generated automatically by Diesel CLI.

diesel::table! {
    soundgasm_profiles (slug) {
        slug -> Text,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
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

diesel::allow_tables_to_appear_in_same_query!(
    soundgasm_profiles,
    soundgasm_tracks,
);
