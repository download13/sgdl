mod download;
mod metadata;
mod pointer;

pub use metadata::TrackMetadata;
pub use pointer::TrackPointer;

pub const TRACK_SLUG_PATTERN: &str = "a-zA-Z0-9_-";
