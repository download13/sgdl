use ratatui::style::{Styled, Stylize};
use ratatui::{
	prelude::Rect,
	widgets::{Block, Paragraph},
	Frame,
};
use tui_input::Input as InputState;
use tuirealm::command::CmdResult;
use tuirealm::{command::Cmd, AttrValue, Attribute, MockComponent, State};

#[derive(Default)]
pub struct LineInput {
	state: InputState,
}

pub impl LineInput {
	pub fn value(&self) -> &str {
		self.state.value()
	}
}

pub impl MockComponent for LineInput {
	fn view(&mut self, frame: &mut Frame, area: Rect) {
		let block = Block::new().style(attr.style).borders(attr.border);

		let block = if let Some(title) = attr.title {
			block.title(title)
		} else {
			block
		};

		let block = if let Some(title) = attr.title_bottom {
			block.title_bottom(title)
		} else {
			block
		};

		let p = Paragraph::new(self.state.value())
			// .wrap(Wrap { trim: true })
			.block(block);

		// let p = if self.focused { p.yellow() } else { p };

		frame.render_widget(p, area);
	}

	fn query(&self, attr: Attribute) -> Option<AttrValue> {}

	fn attr(&mut self, attr: Attribute, value: AttrValue) {}

	fn state(&self) -> State {}

	fn perform(&mut self, cmd: Cmd) -> CmdResult {
		use Cmd::*;

		match cmd {
			Type(ch) => self.0 = self.0.with_value(value),
			_ => {}
		}
	}

	// fn handle_events(&mut self, event: &Event) {
	// 	match event {
	// 		Event::Paste(value) => {
	// 			log::debug!("Paste into LineInput: {}", value);
	// 			self.state = InputState::new(value.clone());
	// 		}
	// 		Event::Key(key) => {
	// 			log::debug!("LineInput KeyEvent: {:?}", key);
	// 			self.state.handle_event(event);
	// 		}
	// 		_ => {}
	// 	}
	// }
}
