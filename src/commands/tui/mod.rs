mod components;

use std::collections::HashMap;

use log::error;
use ratatui::{
	crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
	layout::{Constraint, Layout, Rect},
	style::{Color, Style, Styled, Stylize},
	text::Line,
	widgets::{Block, Borders, Row, Table, TableState},
	DefaultTerminal, Frame,
};
use reqwest::Url;
use tokio::sync::mpsc;
use tui_input::{backend::crossterm::EventHandler, Input};
use unicode_width::UnicodeWidthStr;

use crate::{
	commands::tui::components::{Component, LineInput},
	file_store::download_manager::{DownloadManager, DownloadProgress},
	media_types::MediaItem,
	Context,
};

pub async fn tui_command(context: &mut Context) {
	let terminal = ratatui::init();

	let mut state = TuiState::default();

	if let Err(err) = state.run(terminal, context).await {
		error!("{:#?}", err);
	};

	ratatui::restore();
}

#[derive(Debug, PartialEq, Eq, Default, Copy, Clone)]
enum CyclableInputMode {
	#[default]
	Search,
	List,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum InputMode {
	Cyclable(CyclableInputMode),
	/// Store the previous state while the popup is active
	AddUrl(CyclableInputMode),
}

impl Default for InputMode {
	fn default() -> Self {
		Self::Cyclable(CyclableInputMode::Search)
	}
}

impl InputMode {
	fn next(&self) -> InputMode {
		if let Self::Cyclable(cyclable) = self {
			return InputMode::Cyclable(match cyclable {
				CyclableInputMode::List => CyclableInputMode::Search,
				CyclableInputMode::Search => CyclableInputMode::List,
			});
		}

		*self
	}

	fn prev(&self) -> InputMode {
		self.next()
	}
}

struct TuiState<'a> {
	mode: InputMode,
	search_input: LineInput<'a>,
	add_url_input: Input,
	table_state: TableState,
	download_manager: DownloadManager,
	download_progress: HashMap<Url, DownloadProgress>,
	progress_tx: mpsc::Sender<(Url, DownloadProgress)>,
	receiver_rx: mpsc::Receiver<(Url, DownloadProgress)>,
}

impl<'a> Default for TuiState<'a> {
	fn default() -> Self {
		let (progress_tx, receiver_rx) = mpsc::channel(32);

		Self {
			mode: InputMode::default(),
			search_input: LineInput::default(),
			add_url_input: Input::default(),
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
							}
							_ => {}
						};

						match self.mode {
							InputMode::AddUrl(_) => self.handle_add_url(&event).await,
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

	async fn handle_add_url(&mut self, event: &Event) {
		self.add_url_input.handle_event(event);
	}

	fn render(&mut self, frame: &mut Frame, search_results: &[impl MediaItem]) {
		let area = frame.area();

		let [search_input_area, search_results_area] =
			Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

		self.search_input.draw(frame, search_input_area);

		self.render_search_results(frame, search_results_area, search_results);

		match self.mode {
			InputMode::AddUrl(_) => {
				let popup_area = Rect {
					x: area.width / 4,
					y: area.height / 3,
					width: area.width / 2,
					height: area.height / 3,
				};

				// let input_popup = InputPopup::default()
				// 	.content("fdf")
				// 	.title("Add Url")
				// 	.title_style(Style::new().yellow().bold())
				// 	.border_style(Color::Yellow.into());

				// frame.render_stateful_widget(input_popup, area, &mut self.add_url_state);
			}
			InputMode::Cyclable(_) => {}
		}
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
					"<d>".set_style(Color::Blue),
					" abort download ".into(),
					"<a>".set_style(Color::Blue),
				])
				.centered(),
			),
		)
		.style(style)
		.row_highlight_style(Style::default().fg(Color::Yellow));

		frame.render_widget(list, area);
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
