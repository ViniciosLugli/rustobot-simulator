use super::{
	dashboard::Dashboard,
	editor::{ContentEventData, Editor},
};
use gloo::{console, utils::window};
use yew::prelude::*;
mod features {
	use super::*;

	pub fn calculate_components_division(event: DragEvent) -> Option<f64> {
		if event.client_x() <= 0 && event.client_y() <= 0 {
			return None;
		}
		let window_width = window().inner_width().unwrap().as_f64().unwrap();
		Some((event.client_x() as f64) / window_width * 100.0)
	}

	pub fn handle_division_drag_event(app: &mut App, event: DragEvent) -> bool {
		console::log!(format!("Drag event to position: x: {}, y: {}", event.client_x(), event.client_y()));
		match features::calculate_components_division(event) {
			Some(percentage) => {
				console::log!(format!("Percentage: {}", percentage));
				app.componentes_division_percentage = percentage.min(70.0).max(15.0);
				true
			}
			None => false,
		}
	}
}
pub struct App {
	componentes_division_percentage: f64,
}

pub enum AppMsg {
	DivisionOnDragStart(DragEvent),
	DivisionOnDrag(DragEvent),
	DivisionOnDragEnd(DragEvent),
	EditorOnUpdate(ContentEventData),
}

impl Component for App {
	type Message = AppMsg;
	type Properties = ();

	fn create(_context: &Context<Self>) -> Self {
		Self { componentes_division_percentage: 20.0 }
	}

	fn view(&self, context: &Context<Self>) -> Html {
		html! {
			<div id="app">
				<div id="app__dashboard-container" style={format!("width: {}%", self.componentes_division_percentage)}>
					<Dashboard />
				</div>
				<div id="app__division" draggable="true"
					ondragstart={context.link().callback(|event: DragEvent| AppMsg::DivisionOnDragStart(event))}
					ondrag={context.link().callback(|event: DragEvent| AppMsg::DivisionOnDrag(event))}
					ondragend={context.link().callback(|event: DragEvent| AppMsg::DivisionOnDragEnd(event))}>
				</div>
				<div id="app__editor-container" style={format!("width: {}%", 100.0 - self.componentes_division_percentage)}>
					<Editor on_update={context.link().callback(|event: ContentEventData| AppMsg::EditorOnUpdate(event))} />
				</div>
			</div>
		}
	}

	fn update(&mut self, _context: &Context<Self>, message: Self::Message) -> bool {
		match message {
			AppMsg::DivisionOnDrag(event) | AppMsg::DivisionOnDragEnd(event) => {
				features::handle_division_drag_event(self, event)
			}

			AppMsg::DivisionOnDragStart(event) => {
				event.data_transfer().unwrap().set_drag_image(
					&window().document().unwrap().create_element("div").unwrap(),
					0,
					0,
				);
				features::handle_division_drag_event(self, event)
			}

			AppMsg::EditorOnUpdate(event) => {
				console::log!(&format!("Model updated content: {}", event.model.get_value()));
				console::log!(&format!("Changed data: {:?}", event.changed.changes()));
				false
			}
		}
	}
}
