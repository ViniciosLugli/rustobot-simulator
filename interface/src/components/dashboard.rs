use yew::prelude::*;

#[derive(PartialEq)]
enum RobotDataItemType {
	Position,
	Claw,
}

#[derive(PartialEq, Properties)]
struct RobotDataItemProps {
	pub name: String,
	pub value: String,
	pub item_type: RobotDataItemType,
}

#[function_component]
fn RobotDataItem(props: &RobotDataItemProps) -> Html {
	let RobotDataItemProps { name, value, item_type } = props;

	let class = match item_type {
		RobotDataItemType::Position => "robot-data-item--position",
		RobotDataItemType::Claw => "robot-data-item--claw",
	};

	html! {
		<div class={format!("dashboard__robot-data-positions__item {}", class)}>
			<inline class="dashboard__robot-data-positions__item__name">{name}</inline>
			<div class="dashboard__robot-data-positions__item__value">{value}</div>
		</div>
	}
}

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
				<div id="dashboard__robot-data">
					<div id="dashboard__robot-data-container">
						<RobotDataItem name="X" value="0.0" item_type={RobotDataItemType::Position} />
						<RobotDataItem name="Y" value="0.0"	item_type={RobotDataItemType::Position} />
						<RobotDataItem name="Z" value="0.0" item_type={RobotDataItemType::Position} />
						<RobotDataItem name="CLAW" value="OPEN" item_type={RobotDataItemType::Claw} />
					</div>
				</div>
				<div id="dashboard__robot-controls"></div>
			</div>
		}
	}
}
