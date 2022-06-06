use bevy::{prelude::*, utils::HashMap};
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;
use strum::{EnumIter, EnumProperty, IntoEnumIterator};

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

#[derive(Default)]
struct State {
	slots: HashMap<Slot, Specifier>,
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

fn enter(state: Res<State>, mut slots_changed: EventWriter<SlotChanged>) {
	// On enter, re-initialise entities for all slots
	slots_changed.send_batch(
		state
			.slots
			.iter()
			.map(|(slot, specifier)| SlotChanged(*slot, specifier.clone())),
	);
}

fn update_slot(
	mut commands: Commands,
	mut slots_changed: EventReader<SlotChanged>,
	mut entities: Local<HashMap<Slot, Entity>>,
	asset_server: Res<AssetServer>,
) {
	for SlotChanged(slot, specifier) in slots_changed.iter() {
		// Remove the previous entity in this slot, if any.
		if let Some(entity) = entities.remove(slot) {
			// NOTE: using .add manually rather than the typical .entity, as the entities map will contain stale entities when swapping back to the tool after leaving.
			// TODO: cleaner solution?
			commands.add(DespawnRecursive { entity })
		}

		// TODO: non-equip models
		// TODO: variants (need to check imc?)
		// TODO: Body type (will need to be in state probably) (will need fallback rules) (sounds like an ironworks module thing?)
		// TODO: some IDs don't have an entry for a given slot - how do i surface that error? Might be able to store the handles of the models I load and check the asset's load states (group load state) for errors?
		let mut entity = commands.spawn();
		entity.insert(*slot).with_children(|children| {
			children.spawn_scene(asset_server.load(&format!(
				"iw://chara/equipment/e{0:04}/model/c0101e{0:04}_{1}.mdl",
				specifier.model,
				slot.get_str("suffix").unwrap(),
			)));
		});

		entities.insert(*slot, entity.id());
	}
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

fn exit(mut commands: Commands, query: Query<Entity, With<Slot>>) {
	// TODO: this will probably need to swap to removing some root entity or other marker component when handling character bodies as well
	for entity in query.iter() {
		commands.entity(entity).despawn_recursive();
	}
}
