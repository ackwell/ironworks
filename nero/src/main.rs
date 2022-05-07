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
		// TEMP: web stuff - plugin?
		.add_event::<EventRequestSqpack>()
		.add_system(system_request_sqpack)
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

fn system_ui(
	mut request_sqpack: EventWriter<EventRequestSqpack>,
	mut egui_context: ResMut<EguiContext>,
) {
	// todo: better id
	egui::SidePanel::left("main")
		.resizable(true)
		.show(egui_context.ctx_mut(), |ui| {
			ui.heading("nero");
			if ui.button("sqpack").clicked() {
				request_sqpack.send(EventRequestSqpack)
			}
		});
}

// TODO: name
struct EventRequestSqpack;

// TODO: error handling. should probably put in seperate fn from the main system and unwrap once
//       - looks like i can "chain" systems with In() and such to make an "error handling system"?
fn system_request_sqpack(mut events: EventReader<EventRequestSqpack>) {
	for _event in events.iter() {
		// request_directory().unwrap();
	}
}
