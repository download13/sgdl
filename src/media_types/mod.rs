pub enum MediaType {
	Audio = 0,
	Video = 1,
	Image = 2,
	Text = 3,
	Pdf = 4,
}

pub trait MediaItem {
	pub fn get_source(&self) -> MediaSource;
	pub fn get_file_path(&self) -> String;
	pub async fn try_download(&self) -> bool;
	pub async fn verify_metadata(&self) -> bool;
	pub async fn verify_blob(&self) -> bool;
}
