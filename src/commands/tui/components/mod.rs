mod line_input;
mod popup_input;

use ratatui::{crossterm::event::Event, layout::Rect, Frame};

pub use line_input::LineInput;

pub trait Component {
	type Action;

	fn init(&mut self) {}

	#[allow(unused_variables)]
	fn update(&mut self, action: &Self::Action) {}

	#[allow(unused_variables)]
	fn handle_events(&mut self, event: &Event) {}

	#[allow(unused_variables)]
	fn focused(&mut self, focused: bool) {}

	fn draw(&mut self, frame: &mut Frame, rect: Rect);
}
