use std::fmt::{self, Display};

use super::error::Result;
use crate::sestring::{MacroKind, Payload, SeStr};

pub struct ExtractText<'a>(&'a SeStr, bool);

impl ExtractText<'_> {
	pub(super) fn new(str: &SeStr, use_soft_hyphen: bool) -> ExtractText<'_> {
		ExtractText(str, use_soft_hyphen)
	}

	fn fmt_string(
		str: &SeStr,
		use_soft_hyphen: bool,
		formatter: &mut fmt::Formatter<'_>,
	) -> Result<()> {
		for payload in str.payloads() {
			match payload? {
				Payload::Text(text) => {
					formatter.write_str(text.as_utf8()?)?;
				}
				Payload::Macro(macro_payload) => {
					let c = match macro_payload.kind() {
						MacroKind::NewLine => "\n",
						MacroKind::NonBreakingSpace => "\u{00A0}",
						MacroKind::Hyphen => "-",
						MacroKind::SoftHyphen if use_soft_hyphen => "\u{00AD}",
						_ => {
							continue;
						}
					};
					formatter.write_str(c)?;
				}
			}
		}
		Ok(())
	}
}

impl Display for ExtractText<'_> {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		Self::fmt_string(self.0, self.1, formatter).map_err(|_| fmt::Error)
	}
}
