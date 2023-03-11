use gloo::console;
use js_sys::Date;
use yew::{html, Component, Context, Html};

pub enum Msg {
	Increment,
	Decrement,
}

pub struct App {
	value: i64,
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self { value: 0 }
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Increment => {
				self.value += 1;
				console::log!("plus one");
				true
			}
			Msg::Decrement => {
				self.value -= 1;
				console::log!("minus one");
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		html! {
			<div>
				<div class="panel">
					<button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
						{ "+1" }
					</button>

					<button onclick={ctx.link().callback(|_| Msg::Decrement)}>
						{ "-1" }
					</button>

					<button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
						{ "+1, +1" }
					</button>

				</div>

				<p class="counter">
					{ self.value }
				</p>

				<p class="footer">
					{ "Rendered: " }
					{ String::from(Date::new_0().to_string()) }
				</p>
			</div>
		}
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}
