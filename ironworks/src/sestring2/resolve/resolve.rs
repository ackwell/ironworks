use crate::sestring2::{
	error::Error,
	macro_kind::MacroKind,
	payload::{Expressions, MacroPayload, Payload, TextPayload},
	sestring::SeString,
};

use super::context::Context;

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
		context: &Context,
	) -> Result<String, Error> {
		let mut resolved_payloads = string
			.payloads()
			.map(|payload| self.resolve_payload(payload?, context));

		resolved_payloads.try_fold(String::from(""), |acc, cur| Ok(acc + &cur?))
	}

	fn resolve_payload<'a>(
		&mut self,
		payload: Payload<'a>,
		context: &Context,
	) -> Result<String, Error> {
		match payload {
			Payload::Text(inner) => self.resolve_payload_text(inner, context),
			Payload::Macro(inner) => self.resolve_payload_macro(inner, context),
		}
	}

	fn resolve_payload_text<'a>(
		&mut self,
		payload: TextPayload<'a>,
		context: &Context,
	) -> Result<String, Error> {
		let _ = context;
		Ok(payload.as_utf8()?.to_owned())
	}

	fn resolve_payload_macro<'a>(
		&mut self,
		payload: MacroPayload<'a>,
		context: &Context,
	) -> Result<String, Error> {
		let args = payload.expressions();

		match payload.kind() {
			MacroKind::If => self.resolve_macro_if(args, context),
			MacroKind::NewLine => self.resolve_macro_new_line(args, context),
			MacroKind::ColorType => self.resolve_macro_color_type(args, context),
			MacroKind::EdgeColorType => self.resolve_macro_edge_color_type(args, context),
			other => todo!("unhandled payload kind {other:?}"),
		}
	}

	fn resolve_macro_if(&mut self, args: Expressions, context: &Context) -> Result<String, Error> {
		let (condition, branch_true, branch_false) =
			args.evaluate::<(u32, String, String)>(self, context)?;

		Ok(match condition > 0 {
			true => branch_true,
			false => branch_false,
		})
	}

	fn resolve_macro_new_line(
		&mut self,
		args: Expressions,
		context: &Context,
	) -> Result<String, Error> {
		let _ = (args, context);
		Ok("\n".into())
	}

	fn resolve_macro_color_type(
		&mut self,
		args: Expressions,
		context: &Context,
	) -> Result<String, Error> {
		let _ = (args, context);
		Ok("".into())
	}

	fn resolve_macro_edge_color_type(
		&mut self,
		args: Expressions,
		context: &Context,
	) -> Result<String, Error> {
		let _ = (args, context);
		Ok("".into())
	}
}
