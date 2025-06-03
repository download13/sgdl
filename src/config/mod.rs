#[cfg(not(test))]
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use std::env::current_dir;
use std::path::PathBuf;

#[cfg(test)]
mod test;
#[cfg(test)]
use lazy_static::lazy_static;
#[cfg(test)]
lazy_static! {
	static ref SERVER_HOST: String = test::init_test_server();
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	version: u64,
	pub data_path: PathBuf,
}

impl Config {
	pub fn new() -> Self {
		Self {
			version: 0,
			data_path: Self::get_data_path(),
		}
	}

	#[cfg(not(test))]
	fn get_data_path() -> PathBuf {
		let Some(dirs) = ProjectDirs::from("", "", "sgdl") else {
			panic!("Failed to get project directories");
		};
		let data_path = dirs.data_local_dir().join("sgdl");
		if !data_path.exists() {
			std::fs::create_dir_all(&data_path).unwrap();
		}

		data_path
	}

	#[cfg(test)]
	fn get_data_path() -> PathBuf {
		let data_path = current_dir().unwrap().join("test/tmp");
		if !data_path.exists() {
			std::fs::create_dir_all(&data_path).unwrap();
		}

		data_path
	}

	#[cfg(not(test))]
	pub fn get_server_url(&self) -> String {
		format!("//{}", "soundgasm.net")
	}
	#[cfg(test)]
	pub fn get_server_url(&self) -> String {
		SERVER_HOST.clone()
	}
}

impl std::default::Default for Config {
	fn default() -> Self {
		Self::new()
	}
}
