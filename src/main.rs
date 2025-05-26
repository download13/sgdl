mod commands;
mod common;
mod config;
mod profile;
mod store;
mod track;

use clap::{Parser, Subcommand};
use config::Config;
use std::path::PathBuf;
use store::Store;
use Commands::*;

#[derive(Parser, Debug)]
#[command(name = "sgdl")]
#[command(version)]
struct Cli {
	/// directory to use as storage context
	#[arg(short, long, default_value = None, value_name = "DIR")]
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
	},

	/// TODO: find tracks not in the public profile using a search engine
	#[command(arg_required_else_help = true)]
	AddProfile {
		/// URL or slug of the profile
		profile_id_or_url: String,
	},

	/// add track to the database
	#[command(arg_required_else_help = true)]
	AddTrack {
		/// URL or slug of the profile
		track_url: String,
	},

	/// find tracks locally in the database
	#[command(arg_required_else_help = true)]
	LocalSearch {
		/// search terms
		terms: String,
	},
}

pub struct Context {
	pub config: Config,
	pub store: Store,
}

#[tokio::main]
async fn main() {
	let Ok(config) = confy::load::<Config>("sgdl", None) else {
		eprintln!("Error loading config");
		return;
	};

	let cli = Cli::parse();

	let data_path = match cli.data_path {
		Some(data_path) => data_path,
		None => config.data_path.clone().into(),
	};

	let cmd = cli.command.unwrap();

	let mut store = match Store::new(&data_path).await {
		Ok(store) => store,
		Err(err) => {
			eprintln!("Error initializing store: {:?}", err);
			return;
		}
	};

	let mut context = Context { config, store };

	// TODO: Tagging system
	// TODO: gwasi support
	// TODO: live tag search and create newsfeed

	match cmd {
		Track { track_url } => commands::add_track(context, track_url).await,
		LocalSearch { terms } => {
			println!("searching for {}", terms);
			context.store.search_tracks(terms).await;
		}
		AddProfile { profile_id_or_url } => {
			println!("downloading profile {}", profile_id_or_url);
			commands::add_profile(&mut context, profile_id_or_url).await;
		}
		AddTrack { track_url } => commands::add_profile(&mut context, track_url).await,
		_ => return,
	}
}
