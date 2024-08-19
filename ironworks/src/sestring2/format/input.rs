use std::{borrow::Cow, collections::HashMap};

use super::{
	runtime::{Gender, Player},
	style::{Color, ColorUsage},
	value::Value,
};

#[derive(Debug)]
pub struct Input {
	players: HashMap<u32, Player>,
	local_player: Option<u32>,
	local: HashMap<u32, Value>,
	global: HashMap<u32, Value>,
	colors: HashMap<u32, HashMap<ColorUsage, Color>>,
}

impl Input {
	pub fn new() -> Self {
		Self {
			players: HashMap::new(),
			local_player: None,
			local: HashMap::new(),
			global: HashMap::new(),
			colors: HashMap::new(),
		}
	}

	pub fn add_player(&mut self, id: u32, player: Player) {
		self.players.insert(id, player);
	}

	pub fn set_local_player_id(&mut self, id: u32) {
		self.local_player = Some(id);
	}

	pub fn add_local_parameter(&mut self, id: u32, value: impl Into<Value>) {
		self.local.insert(id, value.into());
	}

	pub fn add_global_parameter(&mut self, id: u32, value: impl Into<Value>) {
		self.global.insert(id, value.into());
	}

	pub fn add_color(&mut self, usage: ColorUsage, id: u32, color: Color) {
		self.colors.entry(id).or_default().insert(usage, color);
	}

	#[must_use]
	pub fn with_player(mut self, id: u32, player: Player) -> Self {
		self.add_player(id, player);
		self
	}

	#[must_use]
	pub fn with_local_player_id(mut self, id: u32) -> Self {
		self.set_local_player_id(id);
		self
	}

	#[must_use]
	pub fn with_local_parameter(mut self, id: u32, value: impl Into<Value>) -> Self {
		self.add_local_parameter(id, value);
		self
	}

	#[must_use]
	pub fn with_global_parameter(mut self, id: u32, value: impl Into<Value>) -> Self {
		self.add_global_parameter(id, value);
		self
	}

	#[must_use]
	pub fn with_color(mut self, usage: ColorUsage, id: u32, color: Color) -> Self {
		self.add_color(usage, id, color);
		self
	}

	// NOTE: marking these as pub(super) for now because I get the sense they'll be moved into a trait.

	pub(super) fn player(&self, id: u32) -> Cow<Player> {
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
