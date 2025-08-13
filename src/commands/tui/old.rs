mod components;

use std::collections::HashMap;

use log::error;
use ratatui::{
	crossterm::event::{
		self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
	},
	layout::{Constraint, Layout, Rect},
	style::{Color, Style, Styled, Stylize},
	text::Line,
	widgets::{Block, Borders, Row, Table, TableState},
	DefaultTerminal, Frame,
};
use reqwest::Url;
use tokio::sync::mpsc;
use tuirealm::{
	application::Application,
	terminal::{TerminalAdapter, TerminalBridge},
	AttrValue, Event, MockComponent, State,
};
use unicode_width::UnicodeWidthStr;

use crate::{
	commands::tui::components::{InputPopup, LineInput},
	file_store::download_manager::{DownloadManager, DownloadProgress},
	media_types::MediaItem,
	Context,
};

const FOCUSED_STYLE: Style = Style::new().fg(Color::Yellow);

pub async fn tui_command(context: &mut Context) {
	let terminal = ratatui::init();

	let mut state = Model::default();

	if let Err(err) = state.run(terminal, context).await {
		error!("{:#?}", err);
	};

	ratatui::restore();
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Id {
	SearchInput,
	TrackList,
	AddUrlPopup,
}

struct Model<T>
where
	T: TerminalAdapter,
{
	/// Application
	pub app: Application<Id, Msg, NoUserEvent>,
	/// Indicates that the application must quit
	pub quit: bool,
	/// Tells whether to redraw interface
	pub redraw: bool,
	/// Used to draw to terminal
	pub terminal: TerminalBridge<T>,
	// mode: InputMode,
	// search_input: LineInput,
	// add_url_input: InputPopup,
	// table_state: TableState,
	// download_manager: DownloadManager,
	// download_progress: HashMap<Url, DownloadProgress>,
	// progress_tx: mpsc::Sender<(Url, DownloadProgress)>,
	// receiver_rx: mpsc::Receiver<(Url, DownloadProgress)>,
}

enum Msg {
	Exit,
}

impl<'a> Default for TuiState<'a> {
	fn default() -> Self {
		let (progress_tx, receiver_rx) = mpsc::channel(32);

		let mut search_input = LineInput::default();
		search_input.focused(true);

		Self {
			mode: InputMode::default(),
			search_input,
			add_url_input: InputPopup::default(),
			table_state: TableState::default(),
			download_manager: DownloadManager::default(),
			download_progress: HashMap::default(),
			progress_tx,
			receiver_rx,
		}
	}
}

impl TuiState<'_> {
	async fn run(&mut self, mut terminal: DefaultTerminal, context: &mut Context) -> Result<(), ()> {
		loop {
			while let Ok((url, progress)) = self.receiver_rx.try_recv() {
				self.download_progress.insert(url, progress);
			}

			let search_results = context.search(self.search_input.value(), None).await;

			let event = match event::read() {
				Ok(event) => event,
				Err(err) => {
					log::error!("Error reading event: {}", err);
					break;
				}
			};

			match event {
				Event::Key(key) => {
					if key.kind == KeyEventKind::Press {
						match key.code {
							KeyCode::Esc => match self.mode {
								InputMode::AddUrl(cyclable) => {
									self.mode = InputMode::Cyclable(cyclable);
								}
								InputMode::Cyclable(_) => {
									break;
								}
							},
							KeyCode::Tab => {
								if key.modifiers.contains(KeyModifiers::SHIFT) {
									self.mode = self.mode.prev();
								} else {
									self.mode = self.mode.next();
								}

								self
									.search_input
									.focused(self.mode == InputMode::Cyclable(CyclableInputMode::Search));
							}
							KeyCode::Char('a') => {
								if key.modifiers.contains(KeyModifiers::CONTROL) {
									if let InputMode::Cyclable(cyclable) = self.mode {
										self.add_url_input.focused(true);
										self.mode = InputMode::AddUrl(cyclable);
									}
								}
							}
							KeyCode::Enter => {
								if let InputMode::AddUrl(cyclable) = self.mode {
									let value = self.add_url_input.value().to_string();
									self.on_popup_submit(value);
									self.add_url_input.focused(false);
									self.mode = InputMode::Cyclable(cyclable);
								}
							}
							_ => {}
						};
						match self.mode {
							InputMode::AddUrl(_) => self.add_url_input.handle_events(&event),
							InputMode::Cyclable(CyclableInputMode::Search) => {
								self.search_input.handle_events(&event);
								self.table_state.select(None);
							}
							InputMode::Cyclable(CyclableInputMode::List) => {
								self.handle_list_keys(key, &search_results).await
							}
						};
					}
				}
				Event::Resize(width, height) => {
					// Handle resize events
					if let Err(err) = terminal.resize(Rect {
						x: 0,
						y: 0,
						width,
						height,
					}) {
						error!("{:#?}", err);
					};
				}
				_ => {}
			}

			if let Err(e) = terminal.draw(|frame| {
				self.render(frame, &search_results);
			}) {
				log::error!("Error drawing terminal: {}", e);
				break;
			}
		}

		log::debug!("User terminated program");

		Ok(())
	}

	async fn handle_list_keys(&mut self, key: KeyEvent, search_results: &[impl MediaItem]) {
		if key.kind == KeyEventKind::Release {
			return;
		}

		match key.code {
			KeyCode::Up => {
				self.table_state.scroll_up_by(1);
			}
			KeyCode::Down => {
				self.table_state.scroll_down_by(1);
			}
			KeyCode::Char('d') => {
				let Some(index) = self.table_state.selected() else {
					return;
				};
				let track = search_results[index].clone();
				self
					.download_manager
					.start_download(track, self.progress_tx.clone())
					.await;
			}
			_ => {}
		}
	}

	fn render(&mut self, frame: &mut Frame, search_results: &[impl MediaItem]) {
		let area = frame.area();

		let [search_input_area, search_results_area] =
			Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

		self.search_input.draw(frame, search_input_area);

		self.render_search_results(frame, search_results_area, search_results);

		match self.mode {
			InputMode::AddUrl(_) => {
				self.add_url_input.focused(true);
				self.add_url_input.draw(frame, area);
			}
			InputMode::Cyclable(_) => {
				self.add_url_input.focused(false);
			}
		}
	}

	fn on_popup_submit(&mut self, value: String) {
		// TODO: Implement what should happen when the popup is submitted
		log::info!("Popup submitted with value: {}", value);
	}

	fn render_search_results(
		&self,
		frame: &mut Frame,
		area: Rect,
		search_results: &[impl MediaItem],
	) {
		let selected = self.mode == InputMode::Cyclable(CyclableInputMode::List);

		let rows = search_results.iter().map(media_item_to_row);

		let (title_width, author_width, source_width, type_width) =
			constraint_len_calculator(search_results);

		let style = match selected {
			true => Color::Yellow.into(),
			false => Style::default(),
		};

		let list = Table::new(
			rows,
			[
				Constraint::Length(title_width + 1),
				Constraint::Min(author_width + 1),
				Constraint::Min(source_width + 1),
				Constraint::Min(type_width),
			],
		)
		.block(
			Block::default().borders(Borders::ALL).title_bottom(
				Line::from(vec![
					"Quit ".into(),
					"<Esc>".blue(),
					" Switch focus ".into(),
					"<Tab>".set_style(Color::Blue),
					" Download selected ".into(),
					"<D>".set_style(Color::Blue),
					" abort download ".into(),
					"<A>".set_style(Color::Blue),
					" Add Track".into(),
					"<Ctrl+A>".set_style(Color::Blue),
				])
				.centered(),
			),
		)
		.style(style)
		.row_highlight_style(Style::default().fg(Color::Yellow));

		frame.render_widget(list, area);
	}

	fn render_cursor(&self) {
		let area = self.get_focused_area();
		let component = self.get_focused_component();

		let scroll = self
			.state
			.visual_scroll(area.width.saturating_sub(1).into());

		// Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
		// end of the input text and one line down from the border to the input line
		let x = self.state.visual_cursor().max(scroll) - scroll + 1;
		frame.set_cursor_position((area.x + x as u16, area.y + 1))
		// TODO: Read about how to structure components
	}

	fn get_focused_area(&self, frame: &Frame) -> Area {
		let area = frame.area();

		let [search_input_area, search_results_area] =
			Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

		match self.mode {
			InputMode::AddUrl(_) => {
				// TODO
			}
			InputMode::Cyclable(CyclableInputMode::Search) => {
				// TODO
			}
			InputMode::Cyclable(CyclableInputMode::List) => {
				// TODO
			}
		}
	}

	fn components_iter() -> Iter<dyn Component> {}

	fn get_focused_component(&self) -> impl Component {
		match self.mode {
			InputMode::AddUrl(_) => self.add_url_input,
			InputMode::Cyclable(CyclableInputMode::Search) => self.search_input,
			InputMode::Cyclable(CyclableInputMode::List) => {
				// TODO: Make search results component
			}
		}
	}
}

fn constraint_len_calculator(items: &[impl MediaItem]) -> (u16, u16, u16, u16) {
	let title_width = items
		.iter()
		.map(|item| item.get_title().width() as u16)
		.max()
		.unwrap_or(0);

	let author_width = items
		.iter()
		.map(|item| item.get_author().width() as u16)
		.max()
		.unwrap_or(0);

	let source_width = items
		.iter()
		.map(|item| item.get_source().to_string().width() as u16)
		.max()
		.unwrap_or(0);

	let type_width = items
		.iter()
		.map(|item| item.get_type().to_string().width() as u16)
		.max()
		.unwrap_or(0);

	(title_width, author_width, source_width, type_width)
}

fn media_item_to_row(item: &impl MediaItem) -> Row<'static> {
	Row::new([
		item.get_title(),
		item.get_author(),
		item.get_source().to_string(),
		item.get_type().to_string(),
	])
}
