use std::path::Path;

pub trait MediaBlob {
	fn get_path(&self) -> &Path;
	fn get_content_hash(&self) -> Option<String>;
	fn get_content_length(&self) -> Option<i64>;
}
