use bevy::{prelude::*, utils::HashMap};
use bevy_egui::{egui, EguiContext};
use ironworks::{EntryKind, ListEntry};
use iyes_loopless::prelude::*;

use crate::asset_io::IronworksResource;

use super::Tool;

pub struct ExplorerTool;
impl Plugin for ExplorerTool {
	fn build(&self, app: &mut App) {
		app.add_system(ui.run_in_state(Some(Tool::Explorer)).label("ui"));
	}
}

fn ui(
	mut egui_context: ResMut<EguiContext>,
	ironworks: Res<IronworksResource>,
	mut list_cache: Local<HashMap<String, Vec<ListEntry>>>,
) {
	let ctx = egui_context.ctx_mut();

	egui::SidePanel::left("explorer")
		.resizable(true)
		.show(ctx, |ui| {
			ui.heading("explorer");
			render_path(ui, "", &ironworks, &mut list_cache);
		});
}

fn render_path(
	ui: &mut egui::Ui,
	path: &str,
	ironworks: &IronworksResource,
	list_cache: &mut HashMap<String, Vec<ListEntry>>,
) {
	let entries = list_cache
		.entry(path.to_string())
		.or_insert_with(|| {
			ironworks
				.read()
				.unwrap()
				.list(path)
				.expect("TODO: error handling")
		})
		// TODO: Can this be avoided? Without it, the recursion leads to nested borrows.
		//       Might able to avoid via sorting folders first...
		.clone();

	for entry in entries {
		match entry.kind {
			EntryKind::Directory => {
				ui.collapsing(format!("🗁 {}", entry.path), |ui| {
					render_path(
						ui,
						format!("{path}/{}", entry.path).trim_start_matches('/'),
						ironworks,
						list_cache,
					)
				});
			}
			EntryKind::File => {
				// Indent to align with header contents. This is done manually rather than .indent to avoid the vertical line.
				ui.horizontal(|ui| {
					ui.add_space(ui.spacing().indent);
					ui.label(format!("🗋 {}", entry.path));
				});
			}
		};
	}
}
