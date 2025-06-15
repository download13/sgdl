use lazy_static::lazy_static;
use regex::Regex;

pub struct TrackDownloadPointer {
	pub sound_id: String,
	pub extension: String,
}

lazy_static! {
	static ref TRACK_DOWNLOAD_RE: Regex =
		Regex::new("//media.soundgasm.net/sounds/([^/]+).([^/]+)").unwrap();
}

impl TrackDownloadPointer {
	pub fn from_html(track_page_html: &String) -> Option<Self> {
		let url_matches = TRACK_DOWNLOAD_RE.captures(track_page_html.as_str())?;

		let sound_id = String::from(url_matches.get(1).unwrap().as_str());
		let extension = String::from(url_matches.get(2).unwrap().as_str());

		Some(Self {
			sound_id,
			extension,
		})
	}

	#[cfg(not(test))]
	pub fn get_download_url(&self) -> String {
		format!(
			"https://media.soundgasm.net/sounds/{}.{}",
			self.sound_id, self.extension
		)
	}

	// TODO: Change this to a mock URL for testing purposes
	#[cfg(test)]
	pub fn get_download_url(&self) -> String {
		format!(
			"https://media.soundgasm.net/sounds/{}.{}",
			self.sound_id, self.extension
		)
	}
}
