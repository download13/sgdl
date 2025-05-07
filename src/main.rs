mod common;
mod profile;
mod store;
mod track;

use clap::{Parser, Subcommand};
use std::{any::Any, future::IntoFuture, path::PathBuf};
use store::Store;
use Commands::*;

#[derive(Parser, Debug)]
#[command(name = "sgdl")]
#[command(version)]
struct Cli {
	/// directory to use as storage context
	#[arg(short, long, default_value = "./data", value_name = "DIR")]
	data_path: Option<PathBuf>,

	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
	/// download a track by it's URL
	#[command(arg_required_else_help = true)]
	Track {
		/// URL of the track to download
		track_url: String,
	},

	/// download all tracks in an author's public profile
	#[command(arg_required_else_help = true)]
	Profile {
		/// URL or slug of the profile
		profile_id_or_url: String,

		/// how many tracks to download at once
		#[arg(short, long, default_value_t = 1)]
		concurrency: u32,

		/// how long to wait between tracks
		#[arg(short, long, default_value_t = 1)]
		wait: u32,
	},
}

#[tokio::main]
async fn main() {
	let cli = Cli::parse();

	let data_path = cli.data_path.unwrap();
	let cmd = cli.command.unwrap();

	match cmd {
		Track { track_url } => track::command(store, track_url),
		/* Profile {
			profile_id_or_url,
			concurrency,
			wait,
		} => profile::command(store, profile_id_or_url, concurrency, wait), */
		_ => (),
	}

	let store = match Store::new(data_path).await {
		Ok(store) => store,
		Err(err) => {
			eprintln!("Error initializing store: {:?}", err);
			return;
		}
	};
}
