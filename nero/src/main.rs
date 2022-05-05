use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		// UI
		.add_plugin(EguiPlugin)
		.insert_resource(WinitSettings::desktop_app())
		.add_system(system_ui)
		// View
		.add_startup_system(startup_test)
		// Done
		.run();
}

fn startup_test(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("icon.png"),
		..default()
	});
}

fn system_ui(mut egui_context: ResMut<EguiContext>) {
	// todo: better id
	egui::SidePanel::left("main")
		.resizable(true)
		.show(egui_context.ctx_mut(), |ui| ui.heading("nero"));
}
