use bevy::{
	prelude::*,
	utils::{HashMap, HashSet},
};
use strum::{EnumProperty, IntoEnumIterator};

use crate::asset_loader::EquipmentDeformerParameter;

use super::shared::{Slot, SlotChanged, State};

pub fn enter(mut slots_changed: EventWriter<SlotChanged>) {
	// On enter, re-initialise entities for all slots
	slots_changed.send_batch(Slot::iter().map(SlotChanged));
}

#[allow(clippy::too_many_arguments)]
pub fn update_slot(
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

pub fn exit(mut commands: Commands, query: Query<Entity, With<Slot>>) {
	// TODO: this will probably need to swap to removing some root entity or other marker component when handling character bodies as well
	for entity in query.iter() {
		commands.entity(entity).despawn_recursive();
	}
}
