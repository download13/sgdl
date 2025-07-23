use ratatui::crossterm::event::Event;
use ratatui::{
	prelude::Rect,
	widgets::{Block, Paragraph},
	Frame,
};
use tui_input::{backend::crossterm::EventHandler, Input as InputState};

use super::Component;

#[derive(Default)]
pub struct LineInput<'a> {
	state: InputState,
	title: Option<&'a str>,
	focused: bool,
}

impl<'a> LineInput<'a> {
	pub fn new(title: Option<&'a str>) -> Self {
		Self {
			state: InputState::default(),
			title,
			focused: false,
		}
	}

	pub fn value(&self) -> &str {
		self.state.value()
	}
}

impl<'a> Component for LineInput<'a> {
	type Action = ();

	fn handle_events(&mut self, event: &Event) {
		if self.focused {
			match event {
				Event::Paste(value) => {
					self.state = InputState::new(value.clone());
				}
				Event::Key(_) => {
					self.state.handle_event(event);
				}
				_ => {}
			}
		}
	}

	fn draw(&mut self, frame: &mut Frame, area: Rect) {
		let block = Block::bordered();

		let block = if let Some(title) = self.title {
			block.title(title)
		} else {
			block
		};

		let p = Paragraph::new(self.state.value())
			// .wrap(Wrap { trim: true })
			.block(block);

		frame.render_widget(p, area);

		if self.focused {
			let scroll = self
				.state
				.visual_scroll(area.width.saturating_sub(1).into());

			// Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
			// end of the input text and one line down from the border to the input line
			let x = self.state.visual_cursor().max(scroll) - scroll + 1;
			frame.set_cursor_position((area.x + x as u16, area.y + 1))
			// TODO: Read about how to structure components
		}
	}
}
