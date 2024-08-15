use super::{
	error::Error,
	expression::Expression,
	macro_kind::MacroKind,
	payload::{Expressions, MacroPayload, Payload, TextPayload},
	sestring::SeString,
};

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
		todo!("impl if")
	}

	fn resolve_macro_new_line(
		&mut self,
		args: Expressions,
		context: &Context,
	) -> Result<String, Error> {
		Ok("\n".into())
	}

	fn resolve_macro_color_type(
		&mut self,
		args: Expressions,
		context: &Context,
	) -> Result<String, Error> {
		Ok("".into())
	}

	fn resolve_macro_edge_color_type(
		&mut self,
		args: Expressions,
		context: &Context,
	) -> Result<String, Error> {
		Ok("".into())
	}
}

// todo maybe "default string"?
pub struct PlainString(());
impl PlainString {
	// todo do i realistically need this? can i afford to use it as a completely argumentless struct?
	pub fn new() -> Self {
		Self(())
	}
}
impl Resolve for PlainString {}

/*
THOUGHTS
- resolve only needs to cater for intepretation - anyone wanting non-interpretive logic can walk themselves with ease
- expressions form the atom
	- they can only represent values of string or numeric types
		- i'll need an unknown value here too
- macros operate on expressions
	- they invariably must "return" either a string, or nothing at all
	- "nothing at all" usually means they mutate state
- sestring is a collection of macros and text
	- can be represented entirely via text

- we don't want someone to reimplement the entire intepretation logic to provide values for expression lookups
	- do we implement that by passing around seperate expression context
		- this seems more tempting, avoids split consumers needing to implement overrides themselves
	- or perhaps inline it all on the trait?

*/

enum Value {
	U32(u32),
	String(String),
	Unknown,
}

pub struct Context(());

impl Context {
	pub fn new() -> Self {
		Self(())
	}
}

impl Context {
	fn evaluate(
		&self,
		expression: Expression,
		resolver: &mut impl Resolve,
	) -> Result<Value, Error> {
		// let res = |expr: Expression<'a>| self.evaluate(expr, resolver);

		let value = match expression {
			Expression::U32(value) => Value::U32(value),
			Expression::SeString(sestring) => {
				Value::String(resolver.resolve_sestring(sestring, self)?)
			}

			Expression::Millisecond => self.unknown(),
			Expression::Second => self.unknown(),
			Expression::Minute => self.unknown(),
			Expression::Hour => self.unknown(),
			Expression::Day => self.unknown(),
			Expression::Weekday => self.unknown(),
			Expression::Month => self.unknown(),
			Expression::Year => self.unknown(),

			// This is effectively a token/fallback value, the expression will be
			// caught before resolution by colour macros.
			Expression::StackColor => self.unknown(),

			Expression::LocalNumber(_expr) => self.unknown(),
			Expression::GlobalNumber(_expr) => self.unknown(),
			Expression::LocalString(_expr) => self.unknown(),
			Expression::GlobalString(_expr) => self.unknown(),

			Expression::Ge(_, _) => todo!(),
			Expression::Gt(_, _) => todo!(),
			Expression::Le(_, _) => todo!(),
			Expression::Lt(_, _) => todo!(),
			Expression::Eq(_, _) => todo!(),
			Expression::Ne(_, _) => todo!(),

			Expression::Unknown(_kind) => Value::Unknown,
		};

		Ok(value)
	}

	fn unknown(&self) -> Value {
		Value::Unknown
	}
}
