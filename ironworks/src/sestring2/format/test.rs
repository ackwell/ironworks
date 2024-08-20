use crate::sestring2::{error::Result, expression::Expression, format, sestring::SeString};

use super::{format::State, time};

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
	Expression::SeString(SeString::new(content))
}

struct TestWriter(String);
impl format::Write for TestWriter {
	fn write_str(&mut self, str: &str) -> Result<()> {
		self.0.push_str(str);
		Ok(())
	}

	fn set_style(&mut self, style: format::Style, enabled: bool) -> Result<()> {
		self.write_str(&format!("[[set_style({style:?}, {enabled:?})]]"))
	}

	fn push_color(&mut self, usage: format::ColorUsage, color: format::Color) -> Result<()> {
		self.write_str(&format!("[[push_color({usage:?}, {color:?})]]"))
	}

	fn pop_color(&mut self, usage: format::ColorUsage) -> Result<()> {
		self.write_str(&format!("[[pop_color({usage:?})]]"))
	}
}
