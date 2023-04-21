use super::convert;

// todo: should probably put the fromstr on it here, but expose it so http can get the format and use it for mime which is http-specific
// TODO: proper tostring impl for this?
#[derive(Debug, Clone, Copy)]
pub enum Format {
	Png,
}

impl Format {
	pub(super) fn converter(&self) -> &dyn convert::Converter {
		match self {
			Self::Png => &convert::Image,
		}
	}
}
