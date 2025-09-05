use xilem::core::adapt;
use xilem::dpi::LogicalSize;
use xilem::view::{button, flex, flex_item, textbox, Axis};
use xilem::winit::window::Window;
use xilem::{EventLoop, WidgetView, Xilem};

struct AppState {
	context: crate::Context,
	search_input: String,
	table_data: Vec<Vec<String>>,
}

enum AppAction {
	UpdateSearchInput(String),
	TableRowClicked(usize),
}

pub fn start_gui(context: &mut crate::Context) {
	let data = AppState {
		context: context.clone(),
		search_input: String::new(),
		table_data: vec![
			vec!["ID".into(), "Name".into(), "Status".into()],
			vec!["1".into(), "Alice".into(), "Active".into()],
			vec!["2".into(), "Bob".into(), "Inactive".into()],
		],
	};

	let app = Xilem::new(data, render_view);
	let event_loop = EventLoop::with_user_event();

	let window_attributes = Window::default_attributes()
		.with_title("SGDL")
		.with_resizable(true)
		.with_min_inner_size(LogicalSize::new(300., 200.))
		.with_inner_size(LogicalSize::new(650., 500.));
	app.run_windowed_in(event_loop, window_attributes).unwrap();
}

fn render_view(data: &mut AppState) -> impl WidgetView<AppState, ()> {
	// TODO
	flex((
		flex_item(
			textbox(data.search_input.clone(), |s: &mut AppState, new_val| {
				s.search_input = new_val;
				println!("Query: {}", s.search_input);
			}),
			1.,
		),
		flex_item(
			button("Click me", |_| {
				println!("Button clicked!");
			}),
			1.,
		),
	))
	.direction(Axis::Vertical)
}
