use derive_setters::Setters;
use ratatui::{
	prelude::{Buffer, Rect},
	style::Style,
	text::{Line, Text},
	widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap},
};
use tui_input::Input;

#[derive(Setters, Default)]
pub struct InputPopup<'a> {
	#[setters(into)]
	title: Line<'a>,
	#[setters(into)]
	content: Text<'a>,
	border_style: Style,
	title_style: Style,
	style: Style,
}

impl StatefulWidget for InputPopup<'_> {
	type State = Input;

	fn render(self, area: Rect, buf: &mut Buffer, value: &mut Input)
	where
		Self: Sized,
	{
		Clear.render(area, buf);

		let block = Block::new()
			.title(self.title)
			.title_style(self.title_style)
			.borders(Borders::ALL)
			.border_style(self.border_style);

		Paragraph::new(self.content)
			.wrap(Wrap { trim: true })
			.style(self.style)
			.block(block)
			.render(area, buf);
	}
}
