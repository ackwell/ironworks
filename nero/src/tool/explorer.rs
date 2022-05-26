use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

use super::Tool;

pub struct ExplorerTool;
impl Plugin for ExplorerTool {
	fn build(&self, app: &mut App) {
		app.add_system(ui.run_in_state(Tool::Explorer).label("ui"));
	}
}

fn ui(mut egui_context: ResMut<EguiContext>) {
	let ctx = egui_context.ctx_mut();

	egui::SidePanel::left("explorer")
		.resizable(true)
		.show(ctx, |ui| {
			ui.heading("explorer");
		});
}
