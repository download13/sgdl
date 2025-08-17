use ratatui::{prelude::Rect, widgets, Frame};
use tuirealm::{Component, Event, MockComponent};

use super::LineInput;

#[derive(Default, MockComponent)]
pub struct PopupInput {
	component: LineInput,
}

// impl Component<Msg, UserEvent> for PopupInput {
// 	fn on(&mut self, ev: Event<UserEvent>) -> Msg {
// 		// TODO
// 	}

// 	fn update(&mut self, action: &Self::Action) {
// 		self.input.update(action);
// 	}

// 	fn handle_events(&mut self, event: &Event) {
// 		self.input.handle_events(event);
// 	}

// 	fn draw(&mut self, frame: &mut Frame, area: Rect, attr: Self::Attr) {
// 		// Center the popup in the area
// 		let popup_width = 40;
// 		let popup_height = 3;
// 		let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
// 		let y = area.y + (area.height.saturating_sub(popup_height)) / 2;
// 		let popup_area = Rect {
// 			x,
// 			y,
// 			width: popup_width,
// 			height: popup_height,
// 		};

// 		// Draw a clear background for the popup
// 		frame.render_widget(widgets::Clear, popup_area);

// 		// Draw the input with a border
// 		self.input.draw(frame, popup_area, attr);
// 	}
// }
