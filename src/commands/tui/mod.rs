mod components;

use std::time::Duration;

use ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::{
	terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge},
	Application, EventListenerCfg, NoUserEvent, PollStrategy, Update,
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

struct Model<'a, T>
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
	context: &'a mut Context,
	// table_state: TableState,
	// download_manager: DownloadManager,
	// download_progress: HashMap<Url, DownloadProgress>,
	// progress_tx: mpsc::Sender<(Url, DownloadProgress)>,
	// receiver_rx: mpsc::Receiver<(Url, DownloadProgress)>,
}

impl<'a> Model<'a, CrosstermTerminalAdapter> {
	fn new(context: &'a mut Context) -> Self {
		let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
			EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 10),
		);

		let search_input = LineInput::default();
		let add_url_input = PopupInput::default();

		assert!(app
			.mount(Id::SearchInput, Box::new(LineInput::default()), vec![])
			.is_ok());

		// We need to give focus to input then
		assert!(app.active(&Id::SearchInput).is_ok());

		Self {
			app: Self::init_app(),
			exit: false,
			redraw: true,
			terminal: TerminalBridge::init_crossterm().expect("Cannot initialize terminal"),
			search_input,
			add_url_input,
			context,
		}
	}
}

impl<'a, T> Model<'a, T>
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
							Constraint::Length(3), // Search Input
							Constraint::Length(3), // Item List
							Constraint::Length(1), // Instructions
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
					LineInput::default() //text("Waiting for a Msg...")
					                     // .alignment(Alignment::Left)
					                     // .background(Color::Reset)
					                     // .foreground(Color::LightYellow)
					                     // .modifiers(Modifier::BOLD),
				),
				Vec::default(),
			)
			.is_ok());

		// Active letter counter
		assert!(app.active(&Id::SearchInput).is_ok());
		app
	}
}

impl<'a, T> Update<Msg> for Model<'a, T>
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
			Msg::SearchUpdate(query) => {
				log::debug!("Search update: {}", query);
				let future = self.context.search(&query, None);
				None
			}
			Msg::AddUrl(url) => {
				self.context.add_url(url);
				None
			}
			Msg::None => None,
		}
	}
}
