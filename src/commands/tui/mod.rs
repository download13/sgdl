use std::collections::HashMap;

use log::error;
use ratatui::{
	crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
	layout::{Constraint, Layout, Rect},
	style::{Color, Style},
	widgets::{Block, Borders, Paragraph, Row, Table, TableState},
	DefaultTerminal, Frame,
};
use reqwest::Url;
use tui_input::{backend::crossterm::EventHandler, Input};
use unicode_width::UnicodeWidthStr;

use crate::{
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

#[derive(PartialEq, Eq, Default)]
enum InputMode {
	#[default]
	Search,
	List,
}

impl InputMode {
	fn next(&self) -> InputMode {
		match self {
			Self::Search => Self::List,
			Self::List => Self::Search,
		}
	}

	fn prev(&self) -> InputMode {
		match self {
			Self::Search => Self::List,
			Self::List => Self::Search,
		}
	}
}

#[derive(Default)]
struct TuiState {
	mode: InputMode,
	search_state: Input,
	table_state: TableState,
	download_progress: HashMap<Url, DownloadProgress>,
}

impl TuiState {
	async fn run(&mut self, mut terminal: DefaultTerminal, context: &mut Context) -> Result<(), ()> {
		let download_manager = DownloadManager::new(32);

		loop {
			let event = match event::read() {
				Ok(event) => event,
				Err(err) => {
					log::error!("Error reading event: {}", err);
					break;
				}
			};

			match event {
				Event::Key(key) => {
					match key.code {
						KeyCode::Esc => break,
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
						InputMode::Search => self.handle_search_keys(key),
						InputMode::List => self.handle_list_keys(key),
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

			let search_results = context.search(self.search_state.value(), None).await;

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

	fn handle_search_keys(&mut self, key: KeyEvent) {
		self.search_state.handle_event(&Event::Key(key));
		self.table_state.select(None);
	}

	fn handle_list_keys(&mut self, key: KeyEvent) {
		match key.code {
			KeyCode::Up => {
				self.table_state.scroll_up_by(1);
			}
			KeyCode::Down => {
				self.table_state.scroll_down_by(1);
			}
			_ => {}
		}
	}

	fn render(&self, frame: &mut Frame, search_results: &[impl MediaItem]) {
		let [search_input_area, search_results_area, instructions_area] = Layout::vertical([
			Constraint::Length(3),
			Constraint::Fill(1),
			Constraint::Length(3),
		])
		.areas(frame.area());

		self.render_search_input(frame, search_input_area);
		self.render_search_results(frame, search_results_area, search_results);
		self.render_instructions(frame, instructions_area);
	}

	fn render_search_input(&self, frame: &mut Frame, area: Rect) {
		let scroll = self
			.search_state
			.visual_scroll(area.width.saturating_sub(1).into());

		let style = match self.mode {
			InputMode::List => Style::default(),
			InputMode::Search => Color::Yellow.into(),
		};

		let input = Paragraph::new(self.search_state.value())
			.style(style)
			.block(Block::bordered().title("Search"));

		frame.render_widget(input, area);

		if self.mode == InputMode::Search {
			// Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
			// end of the input text and one line down from the border to the input line
			let x = self.search_state.visual_cursor().max(scroll) - scroll + 1;
			frame.set_cursor_position((area.x + x as u16, area.y + 1))
		}
	}

	fn render_search_results(
		&self,
		frame: &mut Frame,
		area: Rect,
		search_results: &[impl MediaItem],
	) {
		let rows = search_results.iter().map(media_item_to_row);

		let (title_width, author_width, source_width, type_width) =
			constraint_len_calculator(search_results);

		let style = match self.mode {
			InputMode::Search => Style::default(),
			InputMode::List => Style::default().fg(Color::Yellow),
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
		.block(Block::default().borders(Borders::ALL))
		.style(style)
		.row_highlight_style(Style::default().fg(Color::Yellow));

		frame.render_widget(list, area);
	}

	fn render_instructions(&self, frame: &mut Frame, area: Rect) {
		let instructions = match self.mode {
			InputMode::Search => "<Esc> quit | <Tab> switch focus | type to search",
			InputMode::List => {
				"<Esc> quit | <Tab> switch focus | <d> download selected | <a> abort download  "
			}
		};

		let paragraph = Paragraph::new(instructions)
			.centered()
			.block(Block::default().borders(Borders::ALL));

		frame.render_widget(paragraph, area);
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
