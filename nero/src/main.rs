use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		// UI
		.add_plugin(EguiPlugin)
		.insert_resource(WinitSettings::desktop_app())
		.add_system(system_ui)
		// Done
		.run();
}

fn system_ui(mut egui_context: ResMut<EguiContext>) {
	// todo: better id
	egui::SidePanel::left("main")
		.resizable(true)
		.show(egui_context.ctx_mut(), |ui| ui.heading("nero"));
}
