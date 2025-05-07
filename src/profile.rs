use crate::store::Store;
use std::path::PathBuf;

struct ProfileUrlInfo {
	profile_slug: String,
}

pub fn command(store: Store, profile: String, concurrency: u32, wait: u32) {
	println!("profile {}, {}, {}", profile, concurrency, wait)
}
