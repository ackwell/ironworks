use crate::sestring2::{
	error::Result,
	macro_kind::MacroKind,
	payload::{MacroPayload, Payload, TextPayload},
	sestring::SeString,
};

use super::{
	argument::Arguments, character, context::Context, control_flow, excel, number, shared, text,
	time,
};

#[derive(Debug)]
pub struct DefaultString(());

impl DefaultString {
	// todo do i realistically need this? can i afford to use it as a completely argumentless struct?
	pub fn new() -> Self {
		Self(())
	}
}

impl Resolve for DefaultString {}

pub trait Resolve: Sized {
	fn resolve_sestring<'a>(
		&mut self,
		string: SeString<'a>,
		context: &mut Context,
	) -> Result<String> {
		let mut resolved_payloads = string
			.payloads()
			.map(|payload| self.resolve_payload(payload?, context));

		resolved_payloads.try_fold(String::from(""), |acc, cur| Ok(acc + &cur?))
	}

	fn resolve_payload<'a>(
		&mut self,
		payload: Payload<'a>,
		context: &mut Context,
	) -> Result<String> {
		match payload {
			Payload::Text(inner) => self.resolve_payload_text(inner, context),
			Payload::Macro(inner) => self.resolve_payload_macro(inner, context),
		}
	}

	fn resolve_payload_text<'a>(
		&mut self,
		payload: TextPayload<'a>,
		context: &mut Context,
	) -> Result<String> {
		let _ = context;
		Ok(payload.as_utf8()?.to_owned())
	}

	fn resolve_payload_macro<'a>(
		&mut self,
		payload: MacroPayload<'a>,
		context: &mut Context,
	) -> Result<String> {
		let args = payload.expressions();

		match payload.kind() {
			MacroKind::SetResetTime => self.resolve_macro_set_reset_time(args, context),
			MacroKind::SetTime => self.resolve_macro_set_time(args, context),
			MacroKind::If => self.resolve_macro_if(args, context),
			MacroKind::Switch => self.resolve_macro_switch(args, context),
			MacroKind::PcName => self.resolve_macro_pc_name(args, context),
			MacroKind::IfPcGender => self.resolve_macro_if_pc_gender(args, context),
			MacroKind::IfPcName => self.resolve_macro_if_pc_name(args, context),
			MacroKind::Josa => self.resolve_macro_josa(args, context),
			MacroKind::Josaro => self.resolve_macro_josaro(args, context),
			MacroKind::IfSelf => self.resolve_macro_if_self(args, context),
			MacroKind::NewLine => self.resolve_macro_new_line(args, context),
			MacroKind::Wait => self.resolve_macro_wait(args, context),
			MacroKind::Icon => self.resolve_macro_icon(args, context),
			MacroKind::Color => self.resolve_macro_color(args, context),
			MacroKind::EdgeColor => self.resolve_macro_edge_color(args, context),
			MacroKind::ShadowColor => self.resolve_macro_shadow_color(args, context),
			MacroKind::SoftHyphen => self.resolve_macro_soft_hyphen(args, context),
			MacroKind::Key => self.resolve_macro_key(args, context),
			MacroKind::Scale => self.resolve_macro_scale(args, context),
			MacroKind::Bold => self.resolve_macro_bold(args, context),
			MacroKind::Italic => self.resolve_macro_italic(args, context),
			MacroKind::Edge => self.resolve_macro_edge(args, context),
			MacroKind::Shadow => self.resolve_macro_shadow(args, context),
			MacroKind::NonBreakingSpace => self.resolve_macro_non_breaking_space(args, context),
			MacroKind::Icon2 => self.resolve_macro_icon2(args, context),
			MacroKind::Hyphen => self.resolve_macro_hyphen(args, context),
			MacroKind::Num => self.resolve_macro_num(args, context),
			MacroKind::Hex => self.resolve_macro_hex(args, context),
			MacroKind::Kilo => self.resolve_macro_kilo(args, context),
			MacroKind::Byte => self.resolve_macro_byte(args, context),
			MacroKind::Sec => self.resolve_macro_sec(args, context),
			MacroKind::Time => self.resolve_macro_time(args, context),
			MacroKind::Float => self.resolve_macro_float(args, context),
			MacroKind::Link => self.resolve_macro_link(args, context),
			MacroKind::Sheet => self.resolve_macro_sheet(args, context),
			MacroKind::String => self.resolve_macro_string(args, context),
			MacroKind::Caps => self.resolve_macro_caps(args, context),
			MacroKind::Head => self.resolve_macro_head(args, context),
			MacroKind::Split => self.resolve_macro_split(args, context),
			MacroKind::HeadAll => self.resolve_macro_head_all(args, context),
			MacroKind::Fixed => self.resolve_macro_fixed(args, context),
			MacroKind::Lower => self.resolve_macro_lower(args, context),
			MacroKind::JaNoun => self.resolve_macro_ja_noun(args, context),
			MacroKind::EnNoun => self.resolve_macro_en_noun(args, context),
			MacroKind::DeNoun => self.resolve_macro_de_noun(args, context),
			MacroKind::FrNoun => self.resolve_macro_fr_noun(args, context),
			MacroKind::ChNoun => self.resolve_macro_ch_noun(args, context),
			MacroKind::LowerHead => self.resolve_macro_lower_head(args, context),
			MacroKind::ColorType => self.resolve_macro_color_type(args, context),
			MacroKind::EdgeColorType => self.resolve_macro_edge_color_type(args, context),
			MacroKind::Digit => self.resolve_macro_digit(args, context),
			MacroKind::Ordinal => self.resolve_macro_ordinal(args, context),
			MacroKind::Sound => self.resolve_macro_sound(args, context),
			MacroKind::LevelPos => self.resolve_macro_level_pos(args, context),
			MacroKind::Unknown(UNK) => todo!("unknown macro kind {UNK:x}"),
		}
	}

	fn resolve_macro_set_reset_time<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("SetResetTime")
	}

	fn resolve_macro_set_time<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		time::set_time(self, args, context)
	}

	fn resolve_macro_if<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		control_flow::r#if(self, args, context)
	}

	fn resolve_macro_switch<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		control_flow::switch(self, args, context)
	}

	fn resolve_macro_pc_name<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("PcName")
	}

	fn resolve_macro_if_pc_gender<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("IfPcGender")
	}

	fn resolve_macro_if_pc_name<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("IfPcName")
	}

	fn resolve_macro_josa<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Josa")
	}

	fn resolve_macro_josaro<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Josaro")
	}

	fn resolve_macro_if_self<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("IfSelf")
	}

	fn resolve_macro_new_line<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		character::new_line(self, args, context)
	}

	fn resolve_macro_wait<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		shared::noop(self, args, context)
	}

	fn resolve_macro_icon<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Icon")
	}

	fn resolve_macro_color<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		shared::noop(self, args, context)
	}

	fn resolve_macro_edge_color<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		shared::noop(self, args, context)
	}

	fn resolve_macro_shadow_color<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("ShadowColor")
	}

	fn resolve_macro_soft_hyphen<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		character::soft_hyphen(self, args, context)
	}

	fn resolve_macro_key<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Key")
	}

	fn resolve_macro_scale<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Scale")
	}

	fn resolve_macro_bold<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		shared::noop(self, args, context)
	}

	fn resolve_macro_italic<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		shared::noop(self, args, context)
	}

	fn resolve_macro_edge<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		shared::noop(self, args, context)
	}

	fn resolve_macro_shadow<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		shared::noop(self, args, context)
	}

	fn resolve_macro_non_breaking_space<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		character::non_breaking_space(self, args, context)
	}

	fn resolve_macro_icon2<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Icon2")
	}

	fn resolve_macro_hyphen<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		character::hyphen(self, args, context)
	}

	fn resolve_macro_num<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		number::identity(self, args, context)
	}

	fn resolve_macro_hex<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Hex")
	}

	fn resolve_macro_kilo<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		number::kilo(self, args, context)
	}

	fn resolve_macro_byte<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Byte")
	}

	fn resolve_macro_sec<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		number::sec(self, args, context)
	}

	fn resolve_macro_time<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Time")
	}

	fn resolve_macro_float<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		number::float(self, args, context)
	}

	fn resolve_macro_link<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Link")
	}

	fn resolve_macro_sheet<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		excel::sheet(self, args, context)
	}

	fn resolve_macro_string<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		number::identity(self, args, context)
	}

	fn resolve_macro_caps<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Caps")
	}

	fn resolve_macro_head<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		text::head(self, args, context)
	}

	fn resolve_macro_split<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		text::split(self, args, context)
	}

	fn resolve_macro_head_all<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		text::head_all(self, args, context)
	}

	fn resolve_macro_fixed<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Fixed")
	}

	fn resolve_macro_lower<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		text::lower(self, args, context)
	}

	fn resolve_macro_ja_noun<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		excel::ja_noun(self, args, context)
	}

	fn resolve_macro_en_noun<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		excel::en_noun(self, args, context)
	}

	fn resolve_macro_de_noun<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		excel::de_noun(self, args, context)
	}

	fn resolve_macro_fr_noun<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		excel::fr_noun(self, args, context)
	}

	fn resolve_macro_ch_noun<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		excel::ch_noun(self, args, context)
	}

	fn resolve_macro_lower_head<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		text::lower_head(self, args, context)
	}

	fn resolve_macro_color_type<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		shared::noop(self, args, context)
	}

	fn resolve_macro_edge_color_type<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		shared::noop(self, args, context)
	}

	fn resolve_macro_digit<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		number::digit(self, args, context)
	}

	fn resolve_macro_ordinal<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Ordinal")
	}

	fn resolve_macro_sound<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("Sound")
	}

	fn resolve_macro_level_pos<'a>(
		&mut self,
		args: impl Arguments<'a>,
		context: &mut Context,
	) -> Result<String> {
		todo!("LevelPos")
	}
}
