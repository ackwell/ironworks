use bevy::{
	prelude::*,
	utils::{HashMap, HashSet},
};
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;
use strum::{EnumIter, EnumProperty, IntoEnumIterator};

use crate::asset_loader::EquipmentDeformerParameter;

use super::Tool;

pub struct CharacterTool;
impl Plugin for CharacterTool {
	fn build(&self, app: &mut App) {
		app.init_resource::<State>()
			.add_event::<SlotChanged>()
			.add_enter_system(Some(Tool::Character), enter)
			.add_system(update_slot.run_in_state(Some(Tool::Character)))
			.add_system(ui.run_in_state(Some(Tool::Character)).label("ui"))
			.add_exit_system(Some(Tool::Character), exit);
	}
}

// TODO: Am I able to derive this entirely from excel? CharaMakeType gets most of the way there - but doesn't map to the model IDs we need
#[derive(Clone, Debug)]
struct Character {
	race: Race,
	tribe: Tribe,
	gender: Gender,
	kind: Kind,
}

impl Character {
	fn id(&self) -> u32 {
		use Gender as G;
		use Race as R;
		use Tribe as T;

		let base = match (&self.race, &self.gender, &self.tribe) {
			(R::Hyur, G::Male, T::First) => 101,
			(R::Hyur, G::Female, T::First) => 201,
			(R::Hyur, G::Male, T::Second) => 301,
			(R::Hyur, G::Female, T::Second) => 401,
			(R::Elezen, G::Male, _) => 501,
			(R::Elezen, G::Female, _) => 601,
			(R::Miqote, G::Male, _) => 701,
			(R::Miqote, G::Female, _) => 801,
			(R::Roegadyn, G::Male, _) => 901,
			(R::Roegadyn, G::Female, _) => 1001,
			(R::Lalafell, G::Male, _) => 1101,
			(R::Lalafell, G::Female, _) => 1201,
			(R::AuRa, G::Male, _) => 1301,
			(R::AuRa, G::Female, _) => 1401,
			(R::Hrothgar, G::Male, _) => 1501,
			(R::Hrothgar, G::Female, _) => 1601,
			(R::Viera, G::Male, _) => 1701,
			(R::Viera, G::Female, _) => 1801,
		};

		// NPCs are xxx4, rather than xxx1
		match self.kind {
			Kind::Pc => base,
			Kind::Npc => base + 3,
		}
	}

	fn fallback(&self) -> Option<Self> {
		use Gender as G;
		use Kind as K;
		use Race as R;
		use Tribe as T;

		// Midlander males are the root - they have no fallback.
		if matches!(
			(self.race, self.tribe, self.gender),
			(R::Hyur, T::First, G::Male,)
		) {
			return None;
		}

		// NPCs fall back to their PC counterpart.
		if self.kind == K::Npc {
			return Some(Self {
				kind: K::Pc,
				..self.clone()
			});
		}

		// Hrothgar falls back to Roe.
		if self.race == R::Hrothgar {
			return Some(Self {
				race: R::Roegadyn,
				..self.clone()
			});
		}

		// Midlander and Lala females fall back to their male counterpart.
		if self.gender == G::Female
			&& matches!(
				(self.race, self.tribe),
				(R::Hyur, T::First) | (R::Lalafell, _)
			) {
			return Some(Self {
				gender: G::Male,
				..self.clone()
			});
		}

		// Everything else falls back to Midlander.
		Some(Self {
			race: R::Hyur,
			tribe: T::First,
			..self.clone()
		})
	}
}

impl Default for Character {
	fn default() -> Self {
		Self {
			race: Race::Hyur,
			tribe: Tribe::First,
			gender: Gender::Male,
			kind: Kind::Pc,
		}
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
enum Race {
	Hyur,
	Elezen,
	Miqote,
	Roegadyn,
	Lalafell,
	AuRa,
	Hrothgar,
	Viera,
}

impl Race {
	// TODO: can I use game strings for this?
	fn label(&self) -> &'static str {
		match self {
			Self::Hyur => "Hyur",
			Self::Elezen => "Elezen",
			Self::Miqote => "Miqo'te",
			Self::Roegadyn => "Roegadyn",
			Self::Lalafell => "Lalafell",
			Self::AuRa => "Au Ra",
			Self::Hrothgar => "Hrothgar",
			Self::Viera => "Viera",
		}
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
enum Tribe {
	First,
	Second,
}

impl Tribe {
	fn label(&self, race: Race) -> &'static str {
		match (race, self) {
			(Race::Hyur, Self::First) => "Midlander",
			(Race::Hyur, Self::Second) => "Highlander",
			(Race::Elezen, Self::First) => "Wildwood",
			(Race::Elezen, Self::Second) => "Duskwight",
			(Race::Miqote, Self::First) => "Seeker of the Sun",
			(Race::Miqote, Self::Second) => "Keeper of the Moon",
			(Race::Roegadyn, Self::First) => "Sea Wolf",
			(Race::Roegadyn, Self::Second) => "Hellsguard",
			(Race::Lalafell, Self::First) => "Plainsfolk",
			(Race::Lalafell, Self::Second) => "Dunesfolk",
			(Race::AuRa, Self::First) => "Raen",
			(Race::AuRa, Self::Second) => "Xaela",
			(Race::Hrothgar, Self::First) => "Helions",
			(Race::Hrothgar, Self::Second) => "The Lost",
			(Race::Viera, Self::First) => "Rava",
			(Race::Viera, Self::Second) => "Veena",
		}
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
enum Gender {
	Male,
	Female,
}

impl Gender {
	fn label(&self) -> &'static str {
		match self {
			Self::Male => "Male",
			Self::Female => "Female",
		}
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
enum Kind {
	Pc,
	Npc,
}

#[derive(Default)]
struct State {
	character: Character,
	slots: HashMap<Slot, Specifier>,
}

struct SlotChanged(Slot);

#[derive(Clone, Default)]
struct Specifier {
	// todo what type should these be?
	set: u16,
	// todo weapon type?
	_variant: u16,
	// todo: what about the fourth, is it always 0
}

#[derive(Component, Clone, Copy, Debug, EnumProperty, EnumIter, PartialEq, Eq, Hash)]
enum Slot {
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

fn enter(mut slots_changed: EventWriter<SlotChanged>) {
	// On enter, re-initialise entities for all slots
	slots_changed.send_batch(Slot::iter().map(SlotChanged));
}

#[allow(clippy::too_many_arguments)]
fn update_slot(
	mut commands: Commands,
	mut state: ResMut<State>,
	mut entities: Local<HashMap<Slot, Entity>>,
	asset_server: Res<AssetServer>,

	mut slots_changed: EventReader<SlotChanged>,
	mut pending_slots: Local<HashSet<Slot>>,

	mut character_eqdp: Local<HashMap<u32, Handle<EquipmentDeformerParameter>>>,
	eqdps: Res<Assets<EquipmentDeformerParameter>>,
) {
	// Merge incoming slot changes in with the pending slots.
	for SlotChanged(slot) in slots_changed.iter() {
		pending_slots.insert(*slot);
	}

	// Check each of the changed slots.
	let mut next_pending = Vec::<Slot>::new();
	'slots: for slot in pending_slots.drain() {
		// Grab the specifier for this slot.
		let specifier = state.slots.entry(slot).or_default().clone();

		let mut character = Some(state.character.clone());
		'fallback: while let Some(ref current_character) = character {
			// Get a handle for the current character's eqdp file.
			let handle = character_eqdp
				.entry(current_character.id())
				.or_insert_with(|| {
					asset_server.load(&format!(
						// TODO: also needs to handle the accessory files
						"iw://chara/xls/charadb/equipmentdeformerparameter/c{:04}.eqdp",
						current_character.id(),
					))
				});

			// Resolve the handle to the concrete file - if it's not ready yet, mark
			// this slot to be handled next frame.
			let eqdp = match eqdps.get(handle.clone()) {
				Some(thing) => thing,
				None => {
					next_pending.push(slot);
					continue 'slots;
				}
			};

			// Read slot data for this character's entry for this set's slot.
			let set = eqdp.set(specifier.set);
			let eqdp_slot = match slot {
				Slot::Head => set.head(),
				Slot::Body => set.body(),
				Slot::Gloves => set.hands(),
				Slot::Legs => set.legs(),
				Slot::Feet => set.feet(),
			};

			// If there's a match, we've got a resolved model. Otherwise, drop down
			// to the next fallback and try again.
			match eqdp_slot.model() {
				true => break 'fallback,
				false => character = current_character.fallback(),
			}
		}

		// Remove the previous entity in this slot, if any.
		if let Some(entity) = entities.remove(&slot) {
			// NOTE: using .add manually rather than the typical .entity, as the entities map will contain stale entities when swapping back to the tool after leaving.
			// TODO: cleaner solution?
			commands.add(DespawnRecursive { entity })
		}

		// If there's no matching character at all, stop here - there's no point
		// trying to add a model that won't exist.
		let resolved_character = match character {
			None => continue 'slots,
			Some(a) => a,
		};

		// Build + insert the model entity.
		// TODO: non-equip models
		// TODO: variants (need to check imc?)
		// TODO: some IDs don't have an entry for a given slot - how do i surface that error? Might be able to store the handles of the models I load and check the asset's load states (group load state) for errors?
		let mut entity = commands.spawn();
		entity.insert(slot).with_children(|children| {
			children.spawn_scene(asset_server.load(&format!(
				"iw://chara/equipment/e{0:04}/model/c{1:04}e{0:04}_{2}.mdl",
				specifier.set,
				resolved_character.id(),
				slot.get_str("suffix").unwrap(),
			)));
		});

		entities.insert(slot, entity.id());
	}

	// Processing is complete - merge any slots that were marked to be re-checked
	// next frame into the pending set.
	pending_slots.extend(next_pending);
}

fn ui(
	mut egui_context: ResMut<EguiContext>,
	mut state: ResMut<State>,
	mut slot_changed: EventWriter<SlotChanged>,
) {
	let context = egui_context.ctx_mut();

	egui::SidePanel::left("character")
		.resizable(true)
		.show(context, |ui| {
			ui.heading("character");

			// TODO: This section needs to be pulled out and probably deduped a bunch
			let response = egui::ComboBox::from_id_source("Race")
				.selected_text(state.character.race.label())
				.show_ui(ui, |ui| {
					let mut changed = false;
					for race in Race::iter() {
						changed |= ui
							.selectable_value(&mut state.character.race, race, race.label())
							.changed();
					}
					changed
				});
			let mut changed = response.inner.unwrap_or(false);

			let response = egui::ComboBox::from_id_source("Tribe")
				.selected_text(state.character.tribe.label(state.character.race))
				.show_ui(ui, |ui| {
					let race = state.character.race;
					let mut changed = false;
					for tribe in Tribe::iter() {
						changed |= ui
							.selectable_value(&mut state.character.tribe, tribe, tribe.label(race))
							.changed();
					}
					changed
				});
			changed |= response.inner.unwrap_or(false);

			let response = egui::ComboBox::from_id_source("Gender")
				.selected_text(state.character.gender.label())
				.show_ui(ui, |ui| {
					let mut changed = false;
					for gender in Gender::iter() {
						changed |= ui
							.selectable_value(&mut state.character.gender, gender, gender.label())
							.changed();
					}
					changed
				});
			changed |= response.inner.unwrap_or(false);

			// Skipping kind, do people want to export npc models?

			// Something about the character has changed - mark all slots as potentially changed.
			if changed {
				slot_changed.send_batch(Slot::iter().map(SlotChanged));
			}

			for slot in Slot::iter() {
				let specifier = state.slots.entry(slot).or_default();
				ui.label(slot.get_str("label").unwrap());

				let response = ui.add(egui::DragValue::new(&mut specifier.set).fixed_decimals(0));
				if response.drag_released() || response.lost_focus() {
					slot_changed.send(SlotChanged(slot));
				}
			}
		});
}

fn exit(mut commands: Commands, query: Query<Entity, With<Slot>>) {
	// TODO: this will probably need to swap to removing some root entity or other marker component when handling character bodies as well
	for entity in query.iter() {
		commands.entity(entity).despawn_recursive();
	}
}
