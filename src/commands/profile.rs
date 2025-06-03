#[path = "../profile/mod.rs"]
mod profile;

use crate::profile::{ProfileId, ProfileTrackListing};
use crate::{common::fetch_text, Context};

pub async fn add_profile(context: &mut Context, profile_id_or_url: String) {
	let profile_slug = ProfileId::parse_profile_slug(&profile_id_or_url);

	let profile_url = format!("{}/{}", context.config.get_server_url(), profile_slug);
	let profile_page_html = match fetch_text(profile_url).await {
		Ok(html) => html,
		Err(err) => {
			println!("Error fetching profile page: {}", err);
			return;
		}
	};

	let track_listings = ProfileTrackListing::parse_from_html(&profile_page_html);

	// println!("Profile page HTML: {}", profile_page_html);
	context.store.upsert_track_listings(&track_listings).await;
	// TODO: Show TUI of listings and allow user to select tracks to download
}

// #[cfg(test)]
// mod tests {
// 	use crate::{config::Config, store::Store, Context};

// 	use super::*;

// 	#[tokio::test]
// 	async fn test_get_profile() {
// 		let context = Context {
// 			config: Config::new(),
// 			store: Store::new("test.db".to_string()).await.unwrap(),
// 		};

// 		let track_info = get_profile(context, "evilspic".to_string()).await;
// 		assert!(track_info.is_none());

// 		// Close, but wrong domain
// 		let track_info = TrackId::new(&"//soundgasm.com/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998".to_string());
// 		assert!(track_info.is_none());

// 		// Wrong subdomain
// 		let track_info =
// 			TrackId::new(&"//dfs.soundgasm.net/u/sgdl-test/shopping-mall-half-open-Netherlands-207-AM-161001_0998".to_string());
// 		assert!(track_info.is_none());
// 	}
// }
