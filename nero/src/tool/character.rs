use bevy::{prelude::*, utils::HashMap};
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;
use strum::{EnumIter, EnumProperty, IntoEnumIterator};

use super::Tool;

pub struct CharacterTool;
impl Plugin for CharacterTool {
	fn build(&self, app: &mut App) {
		app.add_event::<SlotChanged>()
			.add_system(update_slot.run_in_state(Some(Tool::Character)))
			.add_system(ui.run_in_state(Some(Tool::Character)).label("ui"));
	}
}

struct SlotChanged(Slot, Specifier);

#[derive(Clone, Default)]
struct Specifier {
	// todo what type should these be?
	model: u16,
	// todo weapon type?
	_variant: u16,
	// todo: what about the fourth, is it always 0
}

#[derive(Component, Clone, Copy, EnumProperty, EnumIter, PartialEq, Eq, Hash)]
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

fn update_slot(
	mut commands: Commands,
	mut slots_changed: EventReader<SlotChanged>,
	// TODO: rather than quering + iterating, maybe store hashmap <slot, entity>?
	entities: Query<(Entity, &Slot)>,
	asset_server: Res<AssetServer>,
) {
	for SlotChanged(slot, specifier) in slots_changed.iter() {
		// TODO: hashmap?
		// Remove the entity currently occupying this slot.
		entities.for_each(|(entity, entity_slot)| {
			if entity_slot != slot {
				return;
			}
			commands.entity(entity).despawn_recursive();
		});

		// TODO: non-equip models
		// TODO: variants (need to check imc?)
		// TODO: Body type (will need to be in state probably) (will need fallback rules) (sounds like an ironworks module thing?)
		// TODO: some IDs don't have an entry for a given slot - how do i surface that error? Might be able to store tha handles of the models I load and check the asset's load states (group load state) for errors?
		commands.spawn().insert(*slot).with_children(|children| {
			children.spawn_scene(asset_server.load(&format!(
				"iw://chara/equipment/e{0:04}/model/c0101e{0:04}_{1}.mdl",
				specifier.model,
				slot.get_str("suffix").unwrap(),
			)));
		});
	}
}

#[derive(Default)]
struct State {
	// TODO: Using a hashmap for sort by key - is that sane? maybe an array/vec?
	slots: HashMap<Slot, Specifier>,
}

fn ui(
	mut egui_context: ResMut<EguiContext>,
	mut state: Local<State>,
	mut slot_changed: EventWriter<SlotChanged>,
) {
	let ctx = egui_context.ctx_mut();

	egui::SidePanel::left("character")
		.resizable(true)
		.show(ctx, |ui| {
			ui.heading("character");

			for slot in Slot::iter() {
				let specifier = state.slots.entry(slot).or_default();
				ui.label(slot.get_str("label").unwrap());

				let response = ui.add(egui::DragValue::new(&mut specifier.model).fixed_decimals(0));
				if response.drag_released() || response.lost_focus() {
					slot_changed.send(SlotChanged(slot, specifier.clone()));
				}
			}
		});
}
