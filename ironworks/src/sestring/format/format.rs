use crate::sestring::{
	error::Result,
	expression::Expression,
	macro_kind::MacroKind,
	payload::{MacroPayload, Payload},
	sestring::SeString,
};

use super::{
	argument::Arguments, character, control_flow, excel, expression::evaluate_expression,
	input::Input, number, runtime, style, text, time, write::Write,
};

#[allow(missing_debug_implementations)]
pub struct State<'a> {
	pub input: &'a Input,
	pub writer: &'a mut dyn Write,

	pub time: u32,
}

pub fn format(sestring: SeString, input: &Input, writer: &mut impl Write) -> Result<()> {
	let mut state = State {
		input,
		writer,
		time: time::FFXIV_EPOCH,
	};
	format_sestring(sestring, &mut state)
}

pub fn format_sestring(sestring: SeString, state: &mut State) -> Result<()> {
	for payload in sestring.payloads() {
		match payload? {
			Payload::Text(inner) => state.writer.write_str(inner.as_utf8()?)?,
			Payload::Macro(inner) => format_macro(inner, state)?,
		}
	}

	Ok(())
}

fn format_macro(payload: MacroPayload, state: &mut State) -> Result<()> {
	let arguments = payload.expressions();

	match payload.kind() {
		MacroKind::SetResetTime => time::set_reset_time(arguments, state),
		MacroKind::SetTime => time::set_time(arguments, state),
		MacroKind::If => control_flow::r#if(arguments, state),
		MacroKind::Switch => control_flow::switch(arguments, state),
		MacroKind::PcName => runtime::pc_name(arguments, state),
		MacroKind::IfPcGender => runtime::if_pc_gender(arguments, state),
		MacroKind::IfPcName => runtime::if_pc_name(arguments, state),
		// Untested, I don't have aceess to a Korean client (or knowledge of how to
		// implement this). Please PR if you can implement these!
		MacroKind::Josa => format_macro_noop(arguments, state),
		MacroKind::Josaro => format_macro_noop(arguments, state),
		MacroKind::IfSelf => runtime::if_self(arguments, state),
		MacroKind::NewLine => character::new_line(arguments, state),
		// Nooping wait because I really don't think that spinning in a formatter is
		// a good idea. If you desperately need this exposed, let me know.
		MacroKind::Wait => format_macro_noop(arguments, state),
		MacroKind::Icon => character::icon(arguments, state),
		MacroKind::Color => style::color(arguments, state),
		MacroKind::EdgeColor => style::edge_color(arguments, state),
		MacroKind::ShadowColor => style::shadow_color(arguments, state),
		MacroKind::SoftHyphen => character::soft_hyphen(arguments, state),
		MacroKind::Key => character::key(arguments, state),
		// Unknown.
		MacroKind::Scale => format_macro_noop(arguments, state),
		MacroKind::Bold => style::bold(arguments, state),
		MacroKind::Italic => style::italic(arguments, state),
		MacroKind::Edge => style::edge(arguments, state),
		MacroKind::Shadow => style::shadow(arguments, state),
		MacroKind::NonBreakingSpace => character::non_breaking_space(arguments, state),
		MacroKind::Icon2 => character::icon(arguments, state),
		MacroKind::Hyphen => character::hyphen(arguments, state),
		MacroKind::Num => format_macro_identity(arguments, state),
		MacroKind::Hex => number::hex(arguments, state),
		MacroKind::Kilo => number::kilo(arguments, state),
		MacroKind::Byte => number::byte(arguments, state),
		MacroKind::Sec => number::sec(arguments, state),
		// Unknown.
		MacroKind::Time => format_macro_noop(arguments, state),
		MacroKind::Float => number::float(arguments, state),
		// I am _not_ in the mood to implement link right now. Unused in excel as of 2024-08-20.
		MacroKind::Link => format_macro_noop(arguments, state),
		MacroKind::Sheet => excel::sheet(arguments, state),
		MacroKind::String => format_macro_identity(arguments, state),
		MacroKind::Caps => text::caps(arguments, state),
		MacroKind::Head => text::head(arguments, state),
		MacroKind::Split => text::split(arguments, state),
		MacroKind::HeadAll => text::head_all(arguments, state),
		// Unknown, only seems to be called with (U32(6), U32(618))
		MacroKind::Fixed => format_macro_noop(arguments, state),
		MacroKind::Lower => text::lower(arguments, state),
		MacroKind::JaNoun => excel::ja_noun(arguments, state),
		MacroKind::EnNoun => excel::en_noun(arguments, state),
		MacroKind::DeNoun => excel::de_noun(arguments, state),
		MacroKind::FrNoun => excel::fr_noun(arguments, state),
		MacroKind::ChNoun => excel::ch_noun(arguments, state),
		MacroKind::LowerHead => text::lower_head(arguments, state),
		MacroKind::ColorType => style::color_type(arguments, state),
		MacroKind::EdgeColorType => style::edge_color_type(arguments, state),
		MacroKind::Ruby => text::ruby(arguments, state),
		MacroKind::Digit => number::digit(arguments, state),
		MacroKind::Ordinal => number::ordinal(arguments, state),
		MacroKind::Sound => excel::sound(arguments, state),
		MacroKind::LevelPos => excel::level_pos(arguments, state),
		// No unknown macros in game data as of 2024/08/18
		MacroKind::Unknown(_kind) => format_macro_noop(arguments, state),
	}
}

fn format_macro_noop<'a>(_arguments: impl Arguments<'a>, _state: &mut State) -> Result<()> {
	Ok(())
}

fn format_macro_identity<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let expression = arguments.exhaustive::<Expression>(state)?;
	format_expression(expression, state)
}

pub fn format_expression(expression: Expression, state: &mut State) -> Result<()> {
	match expression {
		// Immediate SeString branches can be formatted directly.
		Expression::SeString(sestring) => format_sestring(sestring, state),

		// Anything else, try to evaluate it and print as a string.
		other => {
			let string: String = evaluate_expression(other, state)?.into();
			state.writer.write_str(&string)?;
			Ok(())
		}
	}
}
