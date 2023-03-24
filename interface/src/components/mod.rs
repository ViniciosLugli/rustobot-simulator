use yew::{html, Component, Context, Html};

pub mod editor;
use editor::Editor;

pub struct App;

pub enum AppMsg {}

impl Component for App {
	type Message = AppMsg;
	type Properties = ();

	fn create(ctx: &Context<Self>) -> Self {
		Self
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		html! {
			<div id="app">
				<div id="app__editor-container">
					<Editor />
				</div>
				<div id="app__editor-container">
					<Editor />
				</div>
			</div>
		}
	}
}
