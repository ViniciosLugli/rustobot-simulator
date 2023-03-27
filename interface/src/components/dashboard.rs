use yew::{html, Component, Context, Html};

pub struct Dashboard;

pub enum DashboardMsg {}

impl Component for Dashboard {
	type Message = DashboardMsg;
	type Properties = ();

	fn create(_context: &Context<Self>) -> Self {
		Self
	}

	fn view(&self, _context: &Context<Self>) -> Html {
		html! {
			<div id="dashboard">
				<div id="dashboard__robot-data"> {"Robot Data"} </div>
				<div id="dashboard__robot-controls"> {"Robot Controls"} </div>
			</div>
		}
	}
}
