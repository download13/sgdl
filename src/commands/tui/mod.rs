mod download_queue;

use ratatui::{
	crossterm::event::{self, Event, KeyCode},
	layout::{Constraint, Layout, Rect},
	style::{Color, Style},
	widgets::{Block, Borders, List, Paragraph},
	DefaultTerminal, Frame,
};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{media_sources::soundgasm::SoundgasmAudioTrack, media_types::MediaItem, Context};

pub async fn tui_command(context: &mut Context) {
	let terminal = ratatui::init();

	let mut state = TuiState::default();

	state.run(terminal, context);

	ratatui::restore();
}

#[derive(PartialEq, Eq)]
enum InputMode {
	Search,
	List,
}

impl Default for InputMode {
	fn default() -> Self {
		InputMode::Search
	}
}

#[derive(Default)]
struct TuiState {
	search_input: Input,
	mode: InputMode,
}

impl TuiState {
	async fn run(&mut self, mut terminal: DefaultTerminal, context: &mut Context) -> Result<(), ()> {
		let mut search_query = "";

		let search_results = context.search(search_query, None, None).await;

		loop {
			if let Err(e) = terminal.draw(|frame| {
				self.render(frame, &search_results);
			}) {
				eprintln!("Error drawing terminal: {}", e);
				break;
			}

			let event = match event::read() {
				Ok(event) => event,
				Err(e) => {
					eprintln!("Error reading event: {}", e);
					break;
				}
			};

			match event {
				Event::Key(key) => match self.mode {
					InputMode::Search => {
						if key.code == KeyCode::Tab || key.code == KeyCode::Enter {
							self.mode = InputMode::List;
						} else {
							self.search_input.handle_event(&Event::Key(key));
						}
					}
					InputMode::List => {
						if key.code == KeyCode::Char('q') {
							break;
						} else if key.code == KeyCode::Char('s') {
							self.mode = InputMode::Search;
						}
					}
				},
				Event::Resize(width, height) => {
					// Handle resize events
					terminal.resize(Rect {
						x: 0,
						y: 0,
						width,
						height,
					});
				}
				_ => {}
			}
		}

		Ok(())
	}

	fn render(&self, frame: &mut Frame, search_results: &Vec<impl MediaItem>) {
		let [search_input_area, search_results_area, instructions_area] = Layout::vertical([
			Constraint::Length(1),
			Constraint::Length(3),
			Constraint::Min(1),
		])
		.areas(frame.area());

		self.render_search_input(frame, search_input_area);
		// self.render_search_results(frame, search_results_area, search_results);
		self.render_instructions(frame, instructions_area);
	}

	fn render_search_input(&self, frame: &mut Frame, area: Rect) {
		let width = area.height.saturating_sub(3);

		let scroll = self.search_input.visual_scroll(width as usize);

		let style = match self.mode {
			InputMode::List => Style::default(),
			InputMode::Search => Color::Yellow.into(),
		};

		let input = Paragraph::new(self.search_input.value())
			.style(style)
			.scroll((0, scroll as u16))
			.block(Block::bordered().title("Search"));

		frame.render_widget(input, area);

		if self.mode == InputMode::Search {
			// Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
			// end of the input text and one line down from the border to the input line
			let x = self.search_input.visual_cursor().max(scroll) - scroll + 1;
			frame.set_cursor_position((area.x + x as u16, area.y + 1))
		}
	}

	fn render_search_results(
		&self,
		frame: &mut Frame,
		area: Rect,
		search_results: Vec<SoundgasmAudioTrack>,
	) {
		let rows: Vec<_> = search_results
			.iter()
			.map(|track| track.metadata.title.clone())
			.collect();

		let list = List::new(rows)
			.block(Block::default().borders(Borders::ALL))
			.highlight_style(Style::default().fg(Color::Yellow));

		frame.render_widget(list, area);
	}

	fn render_instructions(&self, frame: &mut Frame, area: Rect) {
		let instructions = match self.mode {
			InputMode::Search => "Press <Tab>/<Enter> to switch to list mode",
			InputMode::List => "Press 's'/<Tab> to switch to search mode, <Enter>/<Space> to select items, 'd' to download selected, 'q' to quit",
		};

		let paragraph = Paragraph::new(instructions).block(Block::default().borders(Borders::ALL));

		frame.render_widget(paragraph, area);
	}
}
