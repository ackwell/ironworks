use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::{
	super::Tool,
	shared::{SlotChanged, State},
	ui::ui,
	update::{enter, exit, update_slot},
};

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
