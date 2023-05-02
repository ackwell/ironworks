use std::collections::HashMap;

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

	pub fn integer_parameter(&self, index: u32) -> u32 {
		// This occurs on addon@en:1092/0 - i presume it's a typo, as it looks like it should be referencing param 1.
		if index == 0 {
			return Value::UNKNOWN;
		}

		let raw_index = usize::try_from(index).unwrap() - 1;
		self.integers
			.get(raw_index)
			.copied()
			.unwrap_or(Value::UNKNOWN)
	}

	// TODO: what's the return type? is it always going to be u32 or do i need to return an arbitrary value
	pub fn player_parameter(&self, id: u32) -> u32 {
		match id {
			69 => self.player.level,
			72 => self.player.level,

			// 0: Very unsure on this one. Switches some copy about ending a duty recording?
			// 4: used in addon:102476. it's related to pvp or gc or something. i have no wish to look further.
			// 5: Might be gender related? used in french to switch between some attributive lookups
			// 6: Gender?
			// 7 | 8: seems to be something related to game community tools, is used for fc and pvp team related messages
			// 11 | 12: 11 is an hour-of-the-day value, 12 is minutes of the hour. no idea how these are linked to the player object. xivapi calls them in_game_hours and in_game_minutes
			// 13..=44 | 57..=65: according to xivapi, these are all configured colours? lines up with their use in logmessage i guess
			// 52 | 53 | 54: used in addon:102476. it's related to pvp or gc or something. i have no wish to look further.
			// 66 | 67: Something to do with the french.
			// 68: seems to be related to classjob in some way? xivapi has as classjob_id
			// 70: seemingly used to pick from the 3 starting town consortium NPCs. might be starting city?
			// 71: Race. 3 is lala, not sure about the others.
			// 74: quest/005/ManFst300_00511 (2466) Japanese 112/0 col 1, seems to be related to whether you've met ralph before? possibly NG+ related?
			// 75: Related to controller state. 0 for "gamepad", >0 for "gamepad". addon@1760 suggests 1 might have something to do with xhb?
			// 76: Legacy character status, presumably bool. 1 is legacy, 0 not.
			// 77: Seemingly related to region? used in a date formatting string, 3 formats as D/M/Y, non-3 is formatted as M/D/Y, so i assume 3 is europe.
			// 78: platform. 3 is OSX
			// 79: quest/005/FesVlt102_00515:2/0
			// 80: I _think_ this is boolean of keyboard vs controller state, true is controller?
			// 83: Time... zone? addon:9280/0, uses a if/switch combo to check 0..=5, each of which has identical content but for the set reset time payload.
			// 84..=90: CWLS2..=8 colour?
			// 92: similar to 70, but picks between GC quartermasters. Might be GC affiliation of the player?
			// 94: Something about keyboard?
			_ => Value::UNKNOWN,
		}
	}

	pub fn string_parameter(&self, index: u32) -> String {
		let raw_index = usize::try_from(index).unwrap();
		self.strings.get(raw_index).cloned().unwrap_or("".into())
	}

	pub fn object_parameter(&self, _index: u32) -> String {
		// TODO: I have a funny feeling that they provide _one_ object to the string, and the index in the parameter expression is an offset or index into that object's data. For now, leaving out.
		"".into()
	}
}
