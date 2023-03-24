use super::editor::Editor;
use gloo::console;
use yew::{html, Component, Context, Html};
pub struct App;

mod app_implementation {
	use super::*;

	pub mod editor {
		use super::{
			super::super::editor::{ContentEventData, UpdateCallback},
			*,
		};

		use yew::Callback;

		pub fn get_update_callback() -> Option<UpdateCallback> {
			Some(Callback::from(move |event_data: ContentEventData| {
				console::log!(&format!("Model updated content: {}", event_data.model.get_value()));
				console::log!(&format!("Changed data: {:?}", event_data.changed.changes()));
			}))
		}
	}
}

pub enum AppMsg {}

impl Component for App {
	type Message = AppMsg;
	type Properties = ();

	fn create(_context: &Context<Self>) -> Self {
		Self
	}

	fn view(&self, _context: &Context<Self>) -> Html {
		html! {
			<div id="app">
				<div id="app__editor-container">
					<Editor  update_callback={app_implementation::editor::get_update_callback()} />
				</div>
				<div id="app__editor-container">
					<Editor  update_callback={app_implementation::editor::get_update_callback()} />
				</div>
			</div>
		}
	}
}
