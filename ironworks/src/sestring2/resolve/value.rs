#[derive(Debug)]
pub enum Value {
	U32(u32),
	String(String),
	Unknown,
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

impl From<Value> for String {
	fn from(value: Value) -> Self {
		match value {
			Value::U32(number) => number.to_string(),
			Value::String(string) => string,
			Value::Unknown => todo!("unknown?"),
		}
	}
}
