use monaco::{
	api::{CodeEditorOptions, DisposableClosure, TextModel},
	sys::editor::{BuiltinTheme, IModelContentChangedEvent},
	yew::CodeEditor,
};
use std::rc::Rc;
use yew::{html, Callback, Component, Context, Html, Properties};

const CONTENT_DEFAULT: &str = r#"
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

mod editor_implementation {
	use super::*;

	pub fn get_options() -> CodeEditorOptions {
		CodeEditorOptions::default().with_builtin_theme(BuiltinTheme::VsDark)
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
	pub update_callback: Option<UpdateCallback>,

	#[prop_or(CONTENT_DEFAULT.to_string())]
	pub content: String,
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
			options: Rc::new(editor_implementation::get_options()),
			model,
			_listener: listener,
			update_callback: context.props().update_callback.clone(),
		}
	}

	fn changed(&mut self, _context: &Context<Self>, _old_props: &Self::Properties) -> bool {
		false
	}

	fn view(&self, _context: &Context<Self>) -> Html {
		html! {
			<CodeEditor classes={"code-editor"} options={ self.options.to_sys_options() } model={ self.model.clone() } />
		}
	}

	fn update(&mut self, _context: &Context<Self>, msg: EditorMsg) -> bool {
		match msg {
			EditorMsg::ModelContentChanged(ev) => {
				if let Some(callback) = &self.update_callback {
					callback.emit(ContentEventData { changed: ev, model: self.model.clone() });
				}
				false
			}
		}
	}
}
