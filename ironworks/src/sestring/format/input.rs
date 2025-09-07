use std::{borrow::Cow, collections::HashMap};

use super::{
	runtime::{Gender, Player},
	style::{Color, ColorUsage},
	value::Value,
};

/// Input data for formatting an [`SeString`](crate::sestring::SeString).
///
/// In-game, strings are able to utilise data from a number of sources,
/// including excel sheets, object tables, and parameters provided to the text
/// subsystems. This struct provides a means to emulate this behavior by
/// providing inputs manually.
///
/// By default, most requests for input will return a fallback value.
#[derive(Debug)]
pub struct Input {
	players: HashMap<u32, Player>,
	local_player: Option<u32>,
	local: HashMap<u32, Value>,
	global: HashMap<u32, Value>,
	colors: HashMap<u32, HashMap<ColorUsage, Color>>,
}

impl Input {
	/// Constructs a new `Input` instance with no provided data.
	pub fn new() -> Self {
		Self {
			players: HashMap::new(),
			local_player: None,
			local: HashMap::new(),
			global: HashMap::new(),
			colors: HashMap::new(),
		}
	}

	/// Adds player data at the specified ID within the emulated object table.
	pub fn add_player(&mut self, id: u32, player: Player) {
		self.players.insert(id, player);
	}

	/// Sets the object table ID for the local player. This ID will be used for any
	/// calls to [`add_player`](Self::add_player) that reference the local player
	/// directly.
	pub fn set_local_player_id(&mut self, id: u32) {
		self.local_player = Some(id);
	}

	/// Adds a value to the local parameters array at the specified index. Local
	/// parameters are typically string-specific, representing contextual data
	/// that is relevant to that particular text usage. Local and global
	/// parameters do not share index space.
	pub fn add_local_parameter(&mut self, id: u32, value: impl Into<Value>) {
		self.local.insert(id, value.into());
	}

	/// Adds a value to the global parameters array at the specified index.
	/// In-game, global parameters are used for all string formatting, and have
	/// well-known semantics for each index. Local and global parameters do not
	/// share index space.
	pub fn add_global_parameter(&mut self, id: u32, value: impl Into<Value>) {
		self.global.insert(id, value.into());
	}

	/// Adds a color of the specified usage and id. These colors will be provided
	/// to [`Write`](super::write::Write) implementations when strings call for
	/// their usage. In-game, these values are typically retrieved from the
	/// `UIColor` excel sheet.
	pub fn add_color(&mut self, usage: ColorUsage, id: u32, color: Color) {
		self.colors.entry(id).or_default().insert(usage, color);
	}

	/// Builder-style variant of [`add_player`](Self::add_player).
	#[must_use]
	pub fn with_player(mut self, id: u32, player: Player) -> Self {
		self.add_player(id, player);
		self
	}

	/// Builder-style variant of [`set_local_player_id`](Self::set_local_player_id).
	#[must_use]
	pub fn with_local_player_id(mut self, id: u32) -> Self {
		self.set_local_player_id(id);
		self
	}

	/// Builder-style variant of [`add_local_parameter`](Self::add_local_parameter).
	#[must_use]
	pub fn with_local_parameter(mut self, id: u32, value: impl Into<Value>) -> Self {
		self.add_local_parameter(id, value);
		self
	}

	/// Builder-style variant of [`add_global_parameter`](Self::add_global_parameter).
	#[must_use]
	pub fn with_global_parameter(mut self, id: u32, value: impl Into<Value>) -> Self {
		self.add_global_parameter(id, value);
		self
	}

	/// Builder-style variant of [`add_color`](Self::add_color).
	#[must_use]
	pub fn with_color(mut self, usage: ColorUsage, id: u32, color: Color) -> Self {
		self.add_color(usage, id, color);
		self
	}

	// NOTE: marking these as pub(super) for now because I get the sense they'll be moved into a trait.

	pub(super) fn player(&'_ self, id: u32) -> Cow<'_, Player> {
		// TODO: oncecell the default player?
		self.players.get(&id).map(Cow::Borrowed).unwrap_or_else(|| {
			Cow::Owned(Player {
				name: "Firstname Lastname".into(),
				gender: Gender::Male,
			})
		})
	}

	pub(super) fn local_player_id(&self) -> Option<u32> {
		self.local_player
	}

	pub(super) fn local_parameter(&self, id: u32) -> Value {
		self.local.get(&id).cloned().unwrap_or(Value::Unknown)
	}

	pub(super) fn global_parameter(&self, id: u32) -> Value {
		self.global.get(&id).cloned().unwrap_or(Value::Unknown)
	}

	pub(super) fn color(&self, usage: ColorUsage, id: u32) -> Color {
		self.colors
			.get(&id)
			.and_then(|usages| usages.get(&usage))
			.copied()
			.unwrap_or(
				// magenta as a fallback
				Color {
					r: 255,
					g: 0,
					b: 255,
					a: 255,
				},
			)
	}
}

// Notes on global parameters:
// See also https://github.com/aers/FFXIVClientStructs/blob/36a3e4ce143aa839e9321527e5a825236b0df7fc/FFXIVClientStructs/FFXIV/Component/Text/MacroDecoder.cs#L15
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
