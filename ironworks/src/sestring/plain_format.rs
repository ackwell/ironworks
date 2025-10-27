use std::fmt::{self, Display};

use crate::sestring::{
	SeStr,
	format::{self, PlainWriter},
};

pub struct PlainFormat<'a>(&'a SeStr);

impl PlainFormat<'_> {
	pub(super) fn new(str: &SeStr) -> PlainFormat<'_> {
		PlainFormat(str)
	}
}

impl Display for PlainFormat<'_> {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		format::format(
			self.0,
			&format::Input::new(),
			&mut PlainWriter::new(formatter),
		)
		.map_err(|_| fmt::Error)
	}
}
