use bevy::{prelude::*, utils::HashMap};
use strum::{EnumIter, EnumProperty};

use super::character::Character;

#[derive(Default)]
pub struct State {
	pub character: Character,
	pub slots: HashMap<Slot, Specifier>,
}

// TODO: is this even nessecary? Might be able to just use slot as an event unto itself.
pub struct SlotChanged(pub Slot);

#[derive(Component, Clone, Copy, Debug, EnumProperty, EnumIter, PartialEq, Eq, Hash)]
pub enum Slot {
	#[strum(props(label = "Head", suffix = "met"))]
	Head,
	#[strum(props(label = "Body", suffix = "top"))]
	Body,
	#[strum(props(label = "Gloves", suffix = "glv"))]
	Gloves,
	#[strum(props(label = "Legs", suffix = "dwn"))]
	Legs,
	#[strum(props(label = "Feet", suffix = "sho"))]
	Feet,
}

#[derive(Clone, Default)]
pub struct Specifier {
	// todo what type should these be?
	pub set: u16,
	// todo weapon type?
	_variant: u16,
	// todo: what about the fourth, is it always 0
}
