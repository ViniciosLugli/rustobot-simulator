use monaco::{
	api::{CodeEditorOptions, DisposableClosure, TextModel},
	sys::editor::{BuiltinTheme, IModelContentChangedEvent},
	yew::CodeEditor,
};
use std::rc::Rc;
use yew::prelude::*;

mod features {
	use super::*;

	pub const CONTENT_DEFAULT: &str = r#"
	-- This is a sample Lua script for rustobot-simulator
	function fact (n)
			if n == 0 then
		return 1
		else
			return n * fact(n-1)
		end
	end

	a = 10        -- template number
	print(fact(a))
	"#;

	pub fn get_default_options() -> CodeEditorOptions {
		CodeEditorOptions::default()
			.with_builtin_theme(BuiltinTheme::VsDark)
			.with_automatic_layout(true)
			.with_scroll_beyond_last_line(false)
	}
}

pub struct ContentEventData {
	pub changed: IModelContentChangedEvent,
	pub model: TextModel,
}

pub type UpdateCallback = Callback<ContentEventData>;

pub struct Editor {
	options: Rc<CodeEditorOptions>,
	model: TextModel,
	_listener: DisposableClosure<dyn FnMut(IModelContentChangedEvent)>,
	update_callback: Option<UpdateCallback>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct EditorProps {
	#[prop_or(None)]
	pub on_update: Option<UpdateCallback>,

	#[prop_or(features::CONTENT_DEFAULT.to_string())]
	pub content: String,

	#[prop_or(features::get_default_options())]
	pub options: CodeEditorOptions,
}

pub enum EditorMsg {
	ModelContentChanged(IModelContentChangedEvent),
}

impl Component for Editor {
	type Message = EditorMsg;
	type Properties = EditorProps;

	fn create(context: &Context<Self>) -> Self {
		let model = TextModel::create(&context.props().content, Some("lua"), None).unwrap();
		let callback = context.link().callback(EditorMsg::ModelContentChanged);
		let listener = model.on_did_change_content(move |ev| callback.emit(ev));

		Self {
			options: Rc::new(context.props().options.clone()),
			model,
			_listener: listener,
			update_callback: context.props().on_update.clone(),
		}
	}

	fn changed(&mut self, _context: &Context<Self>, _old_props: &Self::Properties) -> bool {
		false
	}

	fn view(&self, _context: &Context<Self>) -> Html {
		html! {
			<CodeEditor classes={"editor__code-editor"} options={ self.options.to_sys_options() } model={ self.model.clone() } />
		}
	}

	fn update(&mut self, _context: &Context<Self>, message: EditorMsg) -> bool {
		match message {
			EditorMsg::ModelContentChanged(ev) => {
				if let Some(callback) = &self.update_callback {
					callback.emit(ContentEventData { changed: ev, model: self.model.clone() });
				}
				false
			}
		}
	}
}
