use rxtui::prelude::*;

use crate::commands::tui::Msg;

#[derive(Default, MockComponent)]
pub struct LineInput {
	component: Input,
}

impl Component<Msg, NoUserEvent> for LineInput {
	fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
		use tuirealm::command::Direction;

		match ev {
			Event::Keyboard(key) => {
				log::debug!("LineInput KeyEvent: {:?}", key);
				match key.code {
					Key::Char(c) => {
						self.perform(Cmd::Type(c));
					}
					Key::Backspace => {
						self.perform(Cmd::Delete);
					}
					Key::Enter => {
						self.perform(Cmd::Submit);
					}
					Key::Esc => {
						self.perform(Cmd::Cancel);
					}
					Key::Left => {
						self.perform(Cmd::Move(Direction::Left));
					}
					Key::Right => {
						self.perform(Cmd::Move(Direction::Right));
					}
					Key::Up => {
						self.perform(Cmd::Move(Direction::Up));
					}
					Key::Down => {
						self.perform(Cmd::Move(Direction::Down));
					}
					Key::PageUp => {
						self.perform(Cmd::Scroll(Direction::Up));
					}
					Key::PageDown => {
						self.perform(Cmd::Scroll(Direction::Down));
					}
					_ => {}
				}
				None
			}
			Event::Paste(value) => {
				log::debug!("Paste into LineInput: {}", value);
				value.chars().for_each(|c| {
					self.perform(Cmd::Type(c));
				});
				None
			}
			_ => None,
		}
	}
}

// impl MockComponent for LineInput {
// 	fn view(&mut self, frame: &mut Frame, area: Rect) {
// 		let block = Block::new().style().borders(attr.border);

// 		let block = if let Some(title) = attr.title {
// 			block.title(title)
// 		} else {
// 			block
// 		};

// 		let block = if let Some(title) = attr.title_bottom {
// 			block.title_bottom(title)
// 		} else {
// 			block
// 		};

// 		let p = Paragraph::new(self.state.value())
// 			// .wrap(Wrap { trim: true })
// 			.block(block);

// 		// let p = if self.focused { p.yellow() } else { p };

// 		frame.render_widget(p, area);
// 	}

// 	// fn handle_events(&mut self, event: &Event) {
// 	// 	match event {
// 	// 		Event::Paste(value) => {
// 	// 			log::debug!("Paste into LineInput: {}", value);
// 	// 			self.state = InputState::new(value.clone());
// 	// 		}
// 	// 		Event::Key(key) => {
// 	// 			log::debug!("LineInput KeyEvent: {:?}", key);
// 	// 			self.state.handle_event(event);
// 	// 		}
// 	// 		_ => {}
// 	// 	}
// 	// }
// }
