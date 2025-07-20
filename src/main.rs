#![feature(macro_metavar_expr_concat)]

mod commands;
mod common;
mod config;
mod context;
mod file_store;
mod macros;
mod media_sources;
mod media_types;
mod schema;

use clap::{Parser, Subcommand};
use config::Config;
use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::path::Path;
use std::path::PathBuf;
use Commands::*;

use file_store::FileStore;

pub use context::Context;

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
	Tui,
}

impl Clone for Context {
	fn clone(&self) -> Self {
		Self {
			config: self.config.clone(),
			conn: establish_connection(self.file_store.data_path.as_path()),
			file_store: self.file_store.clone(),
		}
	}
}

#[tokio::main]
async fn main() {
	simple_logger::SimpleLogger::new().env().init().unwrap();

	let Ok(config) = confy::load::<Config>("sgdl", None) else {
		eprintln!("Error loading config");
		return;
	};

	let cli = Cli::parse();

	let data_path = match cli.data_path {
		Some(data_path) => data_path,
		None => config.data_path.clone(),
	};

	let cmd = match cli.command {
		Some(cmd) => cmd,
		None => {
			println!("No command provided. Use --help for usage information.");
			return;
		}
	};

	let file_store = FileStore::new(&data_path).await;

	let mut context = Context {
		config,
		file_store,
		conn: establish_connection(&data_path),
	};

	// TODO: Tagging system
	// TODO: gwasi support
	// TODO: live tag search and create newsfeed

	match cmd {
		Scan { media_string } => {
			commands::scan_command(media_string, &mut context).await;
		}
		Tui => {
			commands::tui_command(&mut context).await;
		}
		_ => (),
	};
}

async fn display_progress(total: u64, downloaded: u64) {
	let percentage = (downloaded as f64 / total as f64) * 100.0;
	println!(
		"Downloaded: {} of {} bytes ({:.2}%)",
		downloaded, total, percentage
	);
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn establish_connection(data_path: &Path) -> SqliteConnection {
	let audio_path = data_path.join("audio");
	if !audio_path.exists() {
		std::fs::create_dir_all(&audio_path).unwrap();
	}

	let database_path = data_path.join("data");
	if !database_path.exists() {
		std::fs::create_dir_all(&database_path).unwrap();
	}

	let database_path = database_path.join("meta.sqlite3");

	let mut conn =
		SqliteConnection::establish(database_path.to_str().unwrap()).unwrap_or_else(|err| {
			println!("Error connecting to {err:?}");
			panic!("Error connecting to {}", database_path.display());
		});

	conn
		.run_pending_migrations(MIGRATIONS)
		.unwrap_or_else(|err| {
			println!("Error running migrations: {err:?}");
			panic!("Error running migrations");
		});

	conn
}
