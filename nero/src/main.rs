use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::ironworks::{IronworksAssetIoPlugin, IronworksPlugin, List};

mod ironworks;

fn main() {
	App::new()
		.add_plugins_with(DefaultPlugins, |group| {
			group.add_before::<bevy::asset::AssetPlugin, _>(IronworksAssetIoPlugin)
		})
		// UI
		.add_plugin(EguiPlugin)
		.insert_resource(WinitSettings::desktop_app())
		.add_system(system_ui)
		// View
		.add_startup_system(startup_test)
		// Ironworks
		.add_plugin(IronworksPlugin)
		// Done
		.run();
}

struct TempTestRes {
	handle: Handle<List>,
}

fn startup_test(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("iw://ui/icon/000000/000014_hr1.tex"),
		..default()
	});

	// TODO: realistically this shouldn't be here. just using to test. should it be an entity?
	commands.insert_resource(TempTestRes {
		handle: asset_server.load("iw://exd/root.exl"),
	})
}

fn system_ui(
	mut egui_context: ResMut<EguiContext>,
	temp_test: Res<TempTestRes>,
	lists: Res<Assets<List>>,
) {
	// todo: better id
	egui::SidePanel::left("main")
		.resizable(true)
		.show(egui_context.ctx_mut(), |ui| {
			ui.heading("nero");

			if let Some(List(list)) = lists.get(&temp_test.handle) {
				let text = list.iter().fold(String::new(), |mut acc, cur| {
					acc.reserve(cur.len() + 1);
					acc.push_str(&cur);
					acc.push('\n');
					acc
				});
				ui.label(text);
			}
		});
}
