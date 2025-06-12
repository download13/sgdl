mod commands;
mod common;
mod config;
mod media_sources;
mod media_types;
mod store;

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
	/// catalog media associated with the string provided
	#[command(arg_required_else_help = true)]
	Scan {
		/// URL or other indicator of media to scan
		media_string: String,
	},

	#[command(arg_required_else_help = true)]
	Ensure {
		/// URL or other indicator of media to ensure is downloaded
		media_string: String,
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

	let store = match Store::new(&data_path).await {
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
		Scan { media_string } => {
			commands::scan_command(media_string, &mut context).await;
		}
		// Ensure { media_string } => {
		// 	commands::ensure_command(&mut context, media_string).await;
		// }
		_ => (),
	}
}

async fn display_progress(total: u64, downloaded: u64) {
	let percentage = (downloaded as f64 / total as f64) * 100.0;
	println!(
		"Downloaded: {} of {} bytes ({:.2}%)",
		downloaded, total, percentage
	);
}
