use std::collections::HashMap;

use super::value::Value;

#[derive(Debug)]
pub struct Context {
	// Ambient state
	player_id: u32,

	player_names: HashMap<u32, String>,
	default_name: String,

	// Parameters
	integers: Vec<u32>,
}

impl Default for Context {
	fn default() -> Self {
		Self {
			player_id: Value::UNKNOWN,
			player_names: Default::default(),
			default_name: "Obtaining Signature".into(),
			integers: Default::default(),
		}
	}
}

impl Context {
	pub fn player_id(&self) -> u32 {
		self.player_id
	}

	pub fn player_name(&self, id: u32) -> String {
		self.player_names
			.get(&id)
			.unwrap_or(&self.default_name)
			.clone()
	}

	pub fn integer(&self, index: u32) -> u32 {
		let raw_index = usize::try_from(index).unwrap() - 1;
		// TODO: I'm falling back to 0 when a param isn't available, but I'm not convinced that's the correct approach - realistically this is modelling a string system where arguments are in sync with the string requirements, and a desync would be a failure of some kind. Right? Maybe I should make the fallback u32::MAX and treat that as a "special" value across the baord.
		self.integers
			.get(raw_index)
			.cloned()
			.unwrap_or(Value::UNKNOWN)
	}
}
