mod components;

use std::time::{Duration, SystemTime};

use log::error;
use ratatui::{
	layout::{Alignment, Constraint, Direction, Layout},
	style::{Color, Modifier},
};
use tui_realm_stdlib::{Input, List};
use tuirealm::{
	terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge},
	Application, EventListenerCfg, NoUserEvent, PollStrategy, Sub, Update,
};

use crate::Context;
use components::{LineInput, PopupInput};

pub async fn tui_command(context: &mut Context) {
	let mut model = Model::new(context);

	// let's loop until quit is true
	while !model.exit {
		// Tick
		if let Ok(messages) = model.app.tick(PollStrategy::Once) {
			for msg in messages.into_iter() {
				let mut msg = Some(msg);
				while msg.is_some() {
					msg = model.update(msg);
				}
			}
		}

		// Redraw
		if model.redraw {
			model.view();
			model.redraw = false;
		}
	}

	// Terminate terminal
	let _ = model.terminal.restore();
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Id {
	App,
	SearchInput,
	TrackList,
	Instructions,
	AddUrlPopup,
}

#[derive(PartialEq, Eq, Clone, Hash)]
enum Msg {
	None,
	Exit,
	SearchUpdate(String),
	AddUrl(String),
}

struct Model<T>
where
	T: TerminalAdapter,
{
	/// Application
	app: Application<Id, Msg, NoUserEvent>,
	/// Indicates that the application must quit
	exit: bool,
	/// Tells whether to redraw interface
	redraw: bool,
	/// Used to draw to terminal
	terminal: TerminalBridge<T>,

	search_input: LineInput,
	add_url_input: PopupInput,
	context: Context,
	// table_state: TableState,
	// download_manager: DownloadManager,
	// download_progress: HashMap<Url, DownloadProgress>,
	// progress_tx: mpsc::Sender<(Url, DownloadProgress)>,
	// receiver_rx: mpsc::Receiver<(Url, DownloadProgress)>,
}

impl Model<CrosstermTerminalAdapter> {
	fn new(context: &mut Context) -> Self {
		let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
			EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 10),
		);

		assert!(app
			.mount(Id::SearchInput, Box::new(Input::default()), vec![])
			.is_ok());

		assert!(app
			.mount(Id::TrackList, Box::new(List::default()), vec![])
			.is_ok());

		// We need to give focus to input then
		assert!(app.active(&Id::SearchInput).is_ok());
		Self {
			app: Self::init_app(),
			exit: false,
			redraw: true,
			terminal: TerminalBridge::init_crossterm().expect("Cannot initialize terminal"),
		}
	}
}

impl<T> Model<T>
where
	T: TerminalAdapter,
{
	pub fn view(&mut self) {
		assert!(self
			.terminal
			.draw(|f| {
				let area = f.area();

				let chunks = Layout::default()
					.direction(Direction::Vertical)
					.margin(1)
					.constraints(
						[
							Constraint::Length(3), // Clock
							Constraint::Length(3), // Letter Counter
							Constraint::Length(1), // Label
						]
						.as_ref(),
					)
					.split(area);

				self.app.view(&Id::SearchInput, f, chunks[0]);
				self.app.view(&Id::TrackList, f, chunks[1]);
				self.app.view(&Id::Instructions, f, chunks[2]);

				self.app.view(&Id::AddUrlPopup, f, area);
			})
			.is_ok());
	}

	fn init_app() -> Application<Id, Msg, NoUserEvent> {
		// Setup application
		// NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
		// NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
		// which we will use to update the clock

		let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
			EventListenerCfg::default()
				.crossterm_input_listener(Duration::from_millis(20), 3)
				.poll_timeout(Duration::from_millis(10))
				.tick_interval(Duration::from_secs(1)),
		);

		// Mount components
		assert!(app
			.mount(
				Id::SearchInput,
				Box::new(
					Label::default()
						.//text("Waiting for a Msg...")
						.alignment(Alignment::Left)
						.background(Color::Reset)
						.foreground(Color::LightYellow)
						.modifiers(Modifier::BOLD),
				),
				Vec::default(),
			)
			.is_ok());

		// Mount clock, subscribe to tick
		assert!(app
			.mount(
				Id::Clock,
				Box::new(
					Clock::new(SystemTime::now())
						.alignment(Alignment::Center)
						.background(Color::Reset)
						.foreground(Color::Cyan)
						.modifiers(Modifier::BOLD)
				),
				vec![Sub::new(SubEventClause::Tick, SubClause::Always)]
			)
			.is_ok());

		// Mount counters
		assert!(app
			.mount(
				Id::LetterCounter,
				Box::new(LetterCounter::new(0)),
				Vec::new()
			)
			.is_ok());

		assert!(app
			.mount(
				Id::DigitCounter,
				Box::new(DigitCounter::new(5)),
				Vec::default()
			)
			.is_ok());

		// Active letter counter
		assert!(app.active(&Id::LetterCounter).is_ok());
		app
	}
}

impl<T> Update<Msg> for Model<T>
where
	T: TerminalAdapter,
{
	fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
		self.redraw = true;
		match msg.unwrap_or(Msg::None) {
			Msg::Exit => {
				self.exit = true;
				None
			}
			Msg::SearchUpdate(query) => self.None,
			Msg::GoTo(path) => {
				// Go to and reload tree
				self.scan_dir(path.as_path());
				self.reload_tree();
				None
			}
			Msg::GoToUpperDir => {
				if let Some(parent) = self.upper_dir() {
					self.scan_dir(parent.as_path());
					self.reload_tree();
				}
				None
			}
			Msg::FsTreeBlur => {
				assert!(self.app.active(&Id::GoTo).is_ok());
				None
			}
			Msg::GoToBlur => {
				assert!(self.app.active(&Id::FsTree).is_ok());
				None
			}
			Msg::None => None,
		}
	}
}
