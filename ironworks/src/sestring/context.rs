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

	// Parameters
	integers: Vec<u32>,
}

impl Default for Context {
	fn default() -> Self {
		Self {
			player: Default::default(),
			player_names: Default::default(),
			default_name: "Obtaining Signature".into(),
			integers: Default::default(),
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

	pub fn integer_parameter(&self, index: u32) -> u32 {
		let raw_index = usize::try_from(index).unwrap() - 1;
		// TODO: I'm falling back to 0 when a param isn't available, but I'm not convinced that's the correct approach - realistically this is modelling a string system where arguments are in sync with the string requirements, and a desync would be a failure of some kind. Right? Maybe I should make the fallback u32::MAX and treat that as a "special" value across the baord.
		self.integers
			.get(raw_index)
			.cloned()
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
					ErrorValue::Other("SeString".into()),
					format!("unknown player parameter id {other}"),
				))
			}
		};

		Ok(value)
	}
}
