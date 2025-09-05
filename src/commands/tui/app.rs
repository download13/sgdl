use tuirealm::props::{Alignment, Color, TableBuilder, TextSpan};
use tuirealm::tui::{
	event::KeyCode,
	layout::{Constraint, Direction, Layout},
	widgets::{Block, Borders, Paragraph, Row, Table},
};
use tuirealm::{Component, Event, Frame, MockComponent, NoUserEvent, State, StateValue};

pub struct App {
	search_input: String,
	table_data: Vec<Vec<String>>,
	instructions: String,
	selected_row: usize,
}

impl Default for App {
	fn default() -> Self {
		Self {
			search_input: String::new(),
			table_data: vec![
				vec!["ID".into(), "Name".into(), "Status".into()],
				vec!["1".into(), "Alice".into(), "Active".into()],
				vec!["2".into(), "Bob".into(), "Inactive".into()],
			],
			instructions: "Type to search | ↑/↓ to navigate | Enter to select".into(),
			selected_row: 1,
		}
	}
}

impl MockComponent for App {
	fn view(&mut self, frame: &mut Frame, area: tuirealm::tui::layout::Rect) {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(1)
			.constraints([
				Constraint::Length(3),
				Constraint::Min(5),
				Constraint::Length(2),
			])
			.split(area);

		// Search input
		let search = Paragraph::new(self.search_input.as_ref())
			.block(Block::default().borders(Borders::ALL).title("Search"));
		frame.render_widget(search, chunks[0]);

		// Table
		let header = Row::new(self.table_data[0].clone())
			.style(tuirealm::tui::style::Style::default().fg(Color::Yellow));
		let rows = self.table_data[1..].iter().enumerate().map(|(i, row)| {
			let mut r = Row::new(row.clone());
			if self.selected_row == i + 1 {
				r = r.style(tuirealm::tui::style::Style::default().bg(Color::Blue));
			}
			r
		});
		let table = Table::new(rows)
			.header(header)
			.block(Block::default().borders(Borders::ALL).title("Results"))
			.widths(&[
				Constraint::Length(5),
				Constraint::Length(15),
				Constraint::Length(10),
			]);
		frame.render_widget(table, chunks[1]);

		// Instructions
		let instructions = Paragraph::new(self.instructions.as_ref())
			.alignment(tuirealm::tui::layout::Alignment::Center)
			.block(Block::default().borders(Borders::ALL));
		frame.render_widget(instructions, chunks[2]);
	}

	fn on(&mut self, ev: Event<NoUserEvent>) -> Option<State> {
		match ev {
			Event::Keyboard(key) => match key.code {
				tuirealm::tui::event::KeyCode::Char(c) => {
					self.search_input.push(c);
					Some(State::One(StateValue::String(self.search_input.clone())))
				}
				tuirealm::tui::event::KeyCode::Backspace => {
					self.search_input.pop();
					Some(State::One(StateValue::String(self.search_input.clone())))
				}
				tuirealm::tui::event::KeyCode::Up => {
					if self.selected_row > 1 {
						self.selected_row -= 1;
					}
					Some(State::One(StateValue::Usize(self.selected_row)))
				}
				tuirealm::tui::event::KeyCode::Down => {
					if self.selected_row < self.table_data.len() - 1 {
						self.selected_row += 1;
					}
					Some(State::One(StateValue::Usize(self.selected_row)))
				}
				_ => None,
			},
			_ => None,
		}
	}

	fn state(&self) -> State {
		State::One(StateValue::String(self.search_input.clone()))
	}
}
