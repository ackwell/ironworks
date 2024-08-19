use std::ops::Not;

#[derive(Debug, Clone)]
pub enum Value {
	U32(u32),
	String(String),
	Unknown,
}

impl Value {
	pub const TRUE: Value = Value::U32(1);
	pub const FALSE: Value = Value::U32(0);
}

impl Not for Value {
	type Output = Value;

	fn not(self) -> Self::Output {
		match u32::from(self) {
			0 => Self::TRUE,
			_ => Self::FALSE,
		}
	}
}

impl From<u32> for Value {
	fn from(value: u32) -> Self {
		Self::U32(value)
	}
}

impl From<bool> for Value {
	fn from(value: bool) -> Self {
		match value {
			true => Value::TRUE,
			false => Value::FALSE,
		}
	}
}

impl From<String> for Value {
	fn from(value: String) -> Self {
		Self::String(value)
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

			Value::Unknown => 0,
		}
	}
}

impl From<Value> for bool {
	fn from(value: Value) -> Self {
		match u32::from(value) {
			0 => false,
			_ => true,
		}
	}
}

impl From<Value> for String {
	fn from(value: Value) -> Self {
		match value {
			Value::U32(number) => number.to_string(),
			Value::String(string) => string,
			Value::Unknown => "UNKNOWN".to_string(),
		}
	}
}
