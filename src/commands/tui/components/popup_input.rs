use ratatui::{crossterm::event::Event, prelude::Rect, widgets, Frame};

use crate::commands::tui::components::{Component, LineInput};

#[derive(Default)]
pub struct InputPopup<'a> {
	input: LineInput<'a>,
}

impl<'a> Component for InputPopup<'a> {
	type Action = <LineInput<'a> as Component>::Action;

	fn init(&mut self) {
		self.input.init();
	}

	fn update(&mut self, action: &Self::Action) {
		self.input.update(action);
	}

	fn handle_events(&mut self, event: &Event) {
		self.input.handle_events(event);
	}

	fn focused(&mut self, focused: bool) {
		self.input.focused(focused);
	}

	fn draw(&mut self, frame: &mut Frame, area: Rect) {
		// Center the popup in the area
		let popup_width = 40;
		let popup_height = 3;
		let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
		let y = area.y + (area.height.saturating_sub(popup_height)) / 2;
		let popup_area = Rect {
			x,
			y,
			width: popup_width,
			height: popup_height,
		};

		// Draw a clear background for the popup
		frame.render_widget(widgets::Clear, popup_area);

		// Draw the input with a border
		self.input.draw(frame, popup_area);
	}
}
