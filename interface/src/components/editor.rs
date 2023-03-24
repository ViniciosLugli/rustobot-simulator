use monaco::{api::CodeEditorOptions, sys::editor::BuiltinTheme, yew::CodeEditor};
use std::rc::Rc;
use yew::{html, Component, Context, Html};

const CONTENT: &str = r#"
-- This is a sample Lua script for rustobot-simulator
function fact (n)
		if n == 0 then
	return 1
	else
		return n * fact(n-1)
	end
end

print("enter a number:")
a = io.read("*number")        -- read a number
print(fact(a))
"#;

mod editor_implementation {
	use super::*;

	pub fn get_options() -> CodeEditorOptions {
		CodeEditorOptions::default()
			.with_language("lua".to_owned())
			.with_value(CONTENT.to_owned())
			.with_builtin_theme(BuiltinTheme::VsDark)
	}
}

pub struct Editor {
	options: Rc<CodeEditorOptions>,
}

pub enum EditorMsg {}

impl Component for Editor {
	type Message = EditorMsg;
	type Properties = ();

	fn create(_context: &Context<Self>) -> Self {
		Self { options: Rc::new(editor_implementation::get_options()) }
	}

	fn changed(&mut self, _context: &Context<Self>, _old_props: &Self::Properties) -> bool {
		false
	}

	fn view(&self, _context: &Context<Self>) -> Html {
		html! {
			<CodeEditor classes={"code-editor"} options={ self.options.to_sys_options() } />
		}
	}
}
