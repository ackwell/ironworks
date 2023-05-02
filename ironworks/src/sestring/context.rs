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
	current_time: Option<u32>,

	constants: HashMap<String, u32>,

	// Parameters
	integers: Vec<u32>,
	strings: Vec<String>,
}

impl Default for Context {
	fn default() -> Self {
		Self {
			default_name: "Obtaining Signature".into(),
			time: None,
			current_time: None,

			constants: HashMap::from([
				("AETHERYTE_TEXT_SGL".into(), 0), // Aetheryte text single?
				("ADDONHUD_NAME".into(), 0),      // Targeting Hud sheet
				("QUEST_NAME".into(), 0),
				("WORLDDCGROUPTYPE_NAME".into(), 0),
			]),

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

	pub fn current_time(&self) -> Option<u32> {
		self.current_time
	}

	pub fn constant(&self, name: &str) -> Option<u32> {
		self.constants.get(name).copied()
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
			// TODO: used in addon:102476. it's related to pvp or gc or something. i have no wish to look further.
			4 => Value::UNKNOWN,

			// TODO: similar to 8
			7 => Value::UNKNOWN,
			// TODO: seems to be something related to game community tools, is used for fc and pvp team related messages
			8 => Value::UNKNOWN,

			// TODO: 11 is an hour-of-the-day value, 12 is minutes of the hour. no idea how these are linked to the player object.
			11 => Value::UNKNOWN,
			12 => Value::UNKNOWN,

			// TODO: CWLS 1 colour?
			35 => Value::UNKNOWN,

			// TODO: used in addon:102476. it's related to pvp or gc or something. i have no wish to look further.
			52 | 53 | 54 => Value::UNKNOWN,

			// TODO: seems to be related to classjob in some way?
			68 => Value::UNKNOWN,

			// TODO: what? seems to be level as well, but used exclusively for gatherers?
			69 => self.player.level,

			// TODO: seemingly used to pick from the 3 starting town consortium NPCs. might be starting city?
			70 => Value::UNKNOWN,

			72 => self.player.level,

			// TODO: Related to controller state. 0 for "gamepad", >0 for "gamepad". addon@1760 suggests 1 might have something to do with xhb?
			75 => Value::UNKNOWN,

			// TODO: Seemingly related to region? used in a date formatting string, 3 formats as D/M/Y, non-3 is formatted as M/D/Y, so i assume 3 is europe.
			77 => Value::UNKNOWN,

			// TODO: platform. 3 is OSX
			78 => Value::UNKNOWN,

			// TODO: I _think_ this is boolean of keyboard vs controller state, true is controller?
			80 => Value::UNKNOWN,

			// TODO: Time... zone? addon:9280/0, uses a if/switch combo to check 0..=5, each of which has identical content but for the set reset time payload.
			83 => Value::UNKNOWN,

			// TODO: CWLS2..=8 colour?
			84 | 85 | 86 | 87 | 88 | 89 | 90 => Value::UNKNOWN,

			// TODO: similar to 70, but picks between GC quartermasters. Might be GC affiliation of the player?
			92 => Value::UNKNOWN,

			// TODO: Something about keyboard?
			94 => Value::UNKNOWN,

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

	pub fn object_parameter(&self, _index: u32) -> String {
		// TODO: I have a funny feeling that they provide _one_ object to the string, and the index in the parameter expression is an offset or index into that object's data. For now, leaving out.
		"".into()
	}
}
