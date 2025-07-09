use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct TrackSoundPointer {
	pub sound_id: String,
	pub file_extension: String,
}

lazy_static! {
	static ref TRACK_DOWNLOAD_RE: Regex =
		Regex::new("//media.soundgasm.net/sounds/([^/]+).([^/]+)").unwrap();
}

impl TrackSoundPointer {
	pub fn from_html(track_page_html: &String) -> Option<Self> {
		let url_matches = TRACK_DOWNLOAD_RE.captures(track_page_html.as_str())?;

		let sound_id = String::from(url_matches.get(1).unwrap().as_str());
		let file_extension = String::from(url_matches.get(2).unwrap().as_str());

		Some(Self {
			sound_id,
			file_extension,
		})
	}

	#[cfg(not(test))]
	pub fn get_download_url(&self) -> String {
		format!(
			"https://media.soundgasm.net/sounds/{}.{}",
			self.sound_id, self.file_extension
		)
	}

	// TODO: Change this to a mock URL for testing purposes
	#[cfg(test)]
	pub fn get_download_url(&self) -> String {
		format!(
			"https://media.soundgasm.net/sounds/{}.{}",
			self.sound_id, self.file_extension
		)
	}
}
