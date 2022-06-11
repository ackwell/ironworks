use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use strum::{EnumProperty, IntoEnumIterator};

use super::{
	character::{Gender, Race, Tribe},
	shared::{Slot, SlotChanged, State},
};

pub fn ui(
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
