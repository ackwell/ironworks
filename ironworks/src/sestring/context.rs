use std::collections::HashMap;

use crate::error::{Error, ErrorValue, Result};

use super::value::Value;

#[derive(Debug)]
struct Player {
	id: u32,

	level: u32,
}

impl Default for Player {
	fn default() -> Self {
		Self {
			id: Value::UNKNOWN,
			// Yes, this is the same as Value::UNKNOWN - i'm using MAX for this due to the different semantics.
			level: u32::MAX,
		}
	}
}

#[derive(Debug)]
pub struct Context {
	// Ambient state
	player: Player,

	player_names: HashMap<u32, String>,
	default_name: String,

	time: Option<u32>,

	// Parameters
	integers: Vec<u32>,
	strings: Vec<String>,
}

impl Default for Context {
	fn default() -> Self {
		Self {
			default_name: "Obtaining Signature".into(),
			time: None,

			player: Default::default(),
			player_names: Default::default(),
			integers: Default::default(),
			strings: Default::default(),
		}
	}
}

impl Context {
	pub fn player_id(&self) -> u32 {
		self.player.id
	}

	pub fn player_name(&self, id: u32) -> String {
		self.player_names
			.get(&id)
			.unwrap_or(&self.default_name)
			.clone()
	}

	pub fn time(&self) -> Option<u32> {
		self.time
	}

	pub fn set_time(&mut self, time: u32) {
		self.time = Some(time);
	}

	pub fn integer_parameter(&self, index: u32) -> u32 {
		let raw_index = usize::try_from(index).unwrap() - 1;
		self.integers
			.get(raw_index)
			.copied()
			.unwrap_or(Value::UNKNOWN)
	}

	// TODO: what's the return type? is it always going to be u32 or do i need to return an arbitrary value
	pub fn player_parameter(&self, id: u32) -> Result<u32> {
		let value = match id {
			// TODO: seems to be related to classjob in some way?
			68 => Value::UNKNOWN,

			// TODO: what? seems to be level as well, but used exclusively for gatherers?
			69 => self.player.level,

			72 => self.player.level,

			other => {
				return Err(Error::Invalid(
					ErrorValue::SeString,
					format!("unknown player parameter id {other}"),
				))
			}
		};

		Ok(value)
	}

	pub fn string_parameter(&self, index: u32) -> String {
		let raw_index = usize::try_from(index).unwrap() - 1;
		self.strings.get(raw_index).cloned().unwrap_or("".into())
	}
}
