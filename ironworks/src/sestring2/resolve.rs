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

// pub? this logic will need to be usable by external consumers... will it? they'll be using it via the args thingo
trait TryFromArgument<'a>: Sized {
	fn try_from_argument(
		argument: Option<Expression<'a>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error>;
}

impl<'a> TryFromArgument<'a> for Expression<'a> {
	fn try_from_argument(
		argument: Option<Expression<'a>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error> {
		argument.ok_or_else(|| todo!("error type"))
	}
}

impl TryFromArgument<'_> for Value {
	fn try_from_argument(
		argument: Option<Expression<'_>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error> {
		let expresssion = Expression::try_from_argument(argument, resolver, context)?;
		context.evaluate(expresssion, resolver)
	}
}

// note can't blanket impl these on from/into because it conflicts with the option<T> impl
impl TryFromArgument<'_> for u32 {
	fn try_from_argument(
		argument: Option<Expression<'_>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error> {
		let value = Value::try_from_argument(argument, resolver, context)?;
		Ok(value.into())
	}
}

impl TryFromArgument<'_> for String {
	fn try_from_argument(
		argument: Option<Expression<'_>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error> {
		let value = Value::try_from_argument(argument, resolver, context)?;
		Ok(value.into())
	}
}

impl<'a, T> TryFromArgument<'a> for Option<T>
where
	T: TryFromArgument<'a>,
{
	fn try_from_argument(
		argument: Option<Expression<'a>>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error> {
		Ok(match argument {
			None => None,
			some => Some(T::try_from_argument(some, resolver, context)?),
		})
	}
}

impl From<Value> for u32 {
	fn from(value: Value) -> Self {
		match value {
			Value::U32(number) => number,

			// Falling back to 0 if the parse fails - it seems like SE's number parser
			// is pretty leniant. In some cases there's constants left in the sheet
			// column parameter, all of which invariably end up pointing to column 0.
			Value::String(string) => string.trim().parse::<u32>().unwrap_or(0),

			Value::Unknown => todo!("unknown?"),
		}
	}
}

impl From<Value> for String {
	fn from(value: Value) -> Self {
		match value {
			Value::U32(number) => number.to_string(),
			Value::String(string) => string,
			Value::Unknown => todo!("unknown?"),
		}
	}
}

trait TryFromArguments<'a>: Sized {
	fn try_from_arguments(
		arguments: Expressions<'a>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error>;
}

// this will be implemented with a macro once i do it properly
impl<'a, Arg1: TryFromArgument<'a>, Arg2: TryFromArgument<'a>, Arg3: TryFromArgument<'a>>
	TryFromArguments<'a> for (Arg1, Arg2, Arg3)
{
	fn try_from_arguments(
		mut arguments: Expressions<'a>,
		resolver: &mut impl Resolve,
		context: &Context,
	) -> Result<Self, Error> {
		let result = (
			Arg1::try_from_argument(arguments.next().transpose()?, resolver, context)?,
			Arg2::try_from_argument(arguments.next().transpose()?, resolver, context)?,
			Arg3::try_from_argument(arguments.next().transpose()?, resolver, context)?,
		);

		// todo: check exhausted
		if let Some(_) = arguments.next() {
			todo!("not exhausted")
		}

		Ok(result)
	}
}

impl<'a> Expressions<'a> {
	fn evaluate<T>(self, resolver: &mut impl Resolve, context: &Context) -> Result<T, Error>
	where
		T: TryFromArguments<'a>,
	{
		T::try_from_arguments(self, resolver, context)
	}
}

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
		let mut eval = |expr: Expression| self.evaluate(expr, resolver);

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

			Expression::Ge(left, right) => self.cmp(u32::ge, eval(*left)?, eval(*right)?),
			Expression::Gt(_, _) => todo!(),
			Expression::Le(_, _) => todo!(),
			Expression::Lt(_, _) => todo!(),
			Expression::Eq(left, right) => match self.eq(eval(*left)?, eval(*right)?) {
				true => Value::U32(1),
				false => Value::U32(0),
			},
			Expression::Ne(_, _) => todo!(),

			Expression::Unknown(_kind) => Value::Unknown,
		};

		Ok(value)
	}

	fn cmp(
		&self,
		cmp: impl for<'a, 'b> FnOnce(&'a u32, &'b u32) -> bool,
		left: Value,
		right: Value,
	) -> Value {
		// Unknown is treated as always-successful.
		if matches!(left, Value::Unknown) || matches!(right, Value::Unknown) {
			return Value::U32(1);
		}

		let left = u32::from(left);
		let right = u32::from(right);

		Value::U32(match cmp(&left, &right) {
			true => 1,
			false => 0,
		})
	}

	fn eq(&self, left: Value, right: Value) -> bool {
		match (left, right) {
			// Either side being UNKNOWN is truthy.
			(Value::Unknown, _) | (_, Value::Unknown) => true,
			// If both sides are strings, try to do a string comparison.
			(Value::String(left), Value::String(right)) => left == right,
			// Otherwise, coerce to u32 and compare.
			(left, right) => u32::from(left) == u32::from(right),
		}
	}

	fn unknown(&self) -> Value {
		Value::Unknown
	}
}
