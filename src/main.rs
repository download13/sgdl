mod common;
mod profile;
mod track;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use Commands::*;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
	/// directory to use as storage context
	#[arg(short, long, default_value = ".", value_name = "FILE")]
	data_path: Option<PathBuf>,

	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
	/// download a track by it's URL
	Track {
		/// URL of the track to download
		track_url: String,
	},

	/// download all tracks in an author's public profile
	Profile {
		/// URL or slug of the profile
		profile: String,

		/// how many tracks to download at once
		#[arg(short, long, default_value_t = 1)]
		concurrency: u32,

		/// how long to wait between tracks
		#[arg(short, long, default_value_t = 1)]
		wait: u32,
	},
}

fn main() {
	let cli = Cli::parse();

	let data_path = cli.data_path.unwrap();
	let cmd = cli.command.unwrap();

	match cmd {
		Track { track_url } => track::command(data_path, track_url),

		Profile {
			profile,
			concurrency,
			wait,
		} => profile::command(data_path, profile, concurrency, wait),
	}
}
