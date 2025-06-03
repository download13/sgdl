pub enum MediaSource {
	Soundgasm = 0,
	Kemono = 1,
}

pub enum MediaType {
	Audio = 0,
	Video = 1,
	Image = 2,
	Text = 3,
	Pdf = 4,
}

pub trait MediaItem {
	fn get_file_path(&self) -> String;
	pub async fn try_download(&self) -> bool;
	pub async fn verify_metadata(&self) -> bool;
	pub async fn verify_blob(&self) -> bool;
}
