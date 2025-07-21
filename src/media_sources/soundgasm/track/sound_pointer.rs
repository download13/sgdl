use std::path::PathBuf;

use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Url;

use crate::{media_sources::soundgasm::SoundgasmAudioTrackRow, media_types::MediaBlobPointer};

#[derive(Clone, Debug)]
pub struct TrackSoundPointer {
	pub sound_id: String,
	pub file_extension: String,
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
}

impl MediaBlobPointer for TrackSoundPointer {
	fn get_path(&self) -> PathBuf {
		PathBuf::from(format!("{}.{}", self.sound_id, self.file_extension))
	}

	#[cfg(not(test))]
	fn get_download_url(&self) -> Url {
		Url::parse(
			format!(
				"https://media.soundgasm.net/sounds/{}.{}",
				self.sound_id, self.file_extension
			)
			.as_str(),
		)
		.unwrap()
	}

	// TODO: Change this to a mock URL for testing purposes
	#[cfg(test)]
	fn get_download_url(&self) -> Url {
		Url::parse(
			format!(
				"http://localhost:5268/sounds/{}.{}",
				self.sound_id, self.file_extension
			)
			.as_str(),
		)
		.unwrap()
	}
}

impl TryFrom<&SoundgasmAudioTrackRow> for TrackSoundPointer {
	type Error = String;

	fn try_from(value: &SoundgasmAudioTrackRow) -> Result<Self, Self::Error> {
		let Some(sound_id) = value.sound_id.clone() else {
			return Err("sound_id is None".to_string());
		};

		let Some(file_extension) = value.file_extension.clone() else {
			return Err("file_extension is None".to_string());
		};

		Ok(Self {
			sound_id,
			file_extension,
		})
	}
}

lazy_static! {
	static ref TRACK_DOWNLOAD_RE: Regex =
		Regex::new("//media.soundgasm.net/sounds/([^/.]+).([^\"]+)").unwrap();
}

#[cfg(test)]
mod tests {
	use super::TrackSoundPointer;

	#[test]
	fn test_track_sound_pointer_from_html() {
		let html =
			include_str!("../../../../test/fixtures/http/soundgasm/profiles/sgdl-test/tracks/shopping-mall-half-open-Netherlands-207-AM-161001_0998.html")
				.to_string();

		let pointer = TrackSoundPointer::from_html(&html).unwrap();
		assert_eq!(pointer.sound_id, "7358137704b4386f24c1b5dad8b44fbdb0cf7731");
		assert_eq!(pointer.file_extension, "m4a");
	}
}
