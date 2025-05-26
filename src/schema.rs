// @generated automatically by Diesel CLI.

diesel::table! {
    tracks (profile_slug, track_slug) {
        fetch_url -> Text,
        profile_slug -> Text,
        track_slug -> Text,
        title -> Text,
        description -> Text,
        file_extension -> Text,
        content_hash -> Text,
        content_length -> BigInt,
    }
}
