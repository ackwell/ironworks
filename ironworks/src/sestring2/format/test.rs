use crate::sestring2::{error::Result, expression::Expression, format, sestring::SeString};

use super::{format::State, time};

// todo: try (again) to make a mapped ok version of this
pub fn resolve<'a, F, I>(r#fn: F, input: I) -> String
where
	F: FnOnce(I::IntoIter, &mut State) -> Result<()>,
	I: IntoIterator,
{
	with_state(|state| {
		let arguments = input.into_iter();
		r#fn(arguments, state).expect("test fn should not error");
	})
}

pub fn with_state<F>(r#fn: F) -> String
where
	F: FnOnce(&mut State) -> (),
{
	let mut writer = TestWriter("".into());
	let mut state = State {
		input: &format::Input::new(),
		writer: &mut writer,
		time: time::FFXIV_EPOCH,
	};

	r#fn(&mut state);

	writer.0
}

pub fn str(content: &[u8]) -> Expression {
	Expression::SeString(SeString::from(content))
}

struct TestWriter(String);
impl format::Write for TestWriter {
	fn write(&mut self, str: &str) {
		self.0.push_str(str)
	}

	fn set_style(&mut self, style: format::Style, enabled: bool) {
		self.write(&format!("[[set_style({style:?}, {enabled:?})]]"))
	}

	fn push_color(&mut self, usage: format::ColorUsage, color: format::Color) {
		self.write(&format!("[[push_color({usage:?}, {color:?})]]"))
	}

	fn pop_color(&mut self, usage: format::ColorUsage) {
		self.write(&format!("[[pop_color({usage:?})]]"))
	}
}
