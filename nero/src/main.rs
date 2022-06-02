#![allow(clippy::module_inception)]

mod asset_io;
mod asset_loader;
mod camera;
mod material;
mod render;
mod tool;

use asset_io::{IronworksAssetIoPlugin, IronworksState};
use asset_loader::IronworksPlugin;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use camera::CameraPlugin;
use iyes_loopless::prelude::*;
use material::NeroMaterialPlugin;
use render::RenderPlugin;
use strum::IntoEnumIterator;
use tool::{Tool, ToolPlugins};

fn main() {
	App::new()
		// Ironworks
		.add_plugins_with(DefaultPlugins, |group| {
			group.add_before::<bevy::asset::AssetPlugin, _>(IronworksAssetIoPlugin)
		})
		.add_plugin(IronworksPlugin)
		.add_enter_system(IronworksState::Ready, ironworks_ready)
		// UI
		.insert_resource(WinitSettings::desktop_app())
		.add_plugin(EguiPlugin)
		.add_system(ui_need_ironworks_resource.run_not_in_state(IronworksState::Ready))
		// TODO: the label here should probably be a const somewhere sensible
		// Running before "UI" to ensure it's always outermost.
		.add_system(ui_toolbox.run_in_state(IronworksState::Ready).before("ui"))
		.add_plugin(ToolPlugins)
		// 3D
		.add_plugin(CameraPlugin)
		.add_plugin(RenderPlugin)
		.add_plugin(NeroMaterialPlugin)
		// Done
		.run();
}

fn ironworks_ready(mut commands: Commands) {
	commands.insert_resource(NextState(Some(Tool::iter().next().unwrap())));
}

fn ui_need_ironworks_resource(
	mut commands: Commands,
	mut egui_context: ResMut<EguiContext>,
	ironworks_state: Res<CurrentState<IronworksState>>,
) {
	let pending = *ironworks_state == CurrentState(IronworksState::ResourceRequested);

	egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
		ui.vertical_centered(|ui| {
			ui.heading("nero");

			// TODO: Work out how to show errors from path validation.
			ui.label("Could not find game installation path.");

			if ui
				.add_enabled(!pending, egui::Button::new("Select game folder"))
				.clicked()
			{
				commands.insert_resource(NextState(IronworksState::ResourceRequested));
			}
		})
	});
}

#[derive(Default)]
struct ToolState {
	icons: Option<Vec<(Handle<Image>, egui::TextureId)>>,
}

fn ui_toolbox(
	mut commands: Commands,
	mut egui_context: ResMut<EguiContext>,
	mut tool_state: Local<ToolState>,
	asset_server: Res<AssetServer>,
	current_tool: Res<CurrentState<Option<Tool>>>,
) {
	// Get the icons for tools, creating them if they don't exist yet.
	let tool_icons = tool_state.icons.get_or_insert_with(|| {
		Tool::iter()
			.map(|tool| {
				let handle = asset_server.load(tool.icon());
				let id = egui_context.add_image(handle.clone());
				(handle, id)
			})
			.collect()
	});

	let ctx = egui_context.ctx_mut();

	// Render the primary toolbox along the left side.
	egui::SidePanel::left("toolbox")
		.width_range(20.0..=20.0)
		.resizable(false)
		.frame(egui::Frame::default().fill(ctx.style().visuals.window_fill()))
		.show(ctx, |ui| {
			for (index, tool) in Tool::iter().enumerate() {
				let selected = matches!(&*current_tool, CurrentState(Some(t)) if t == &tool);
				let response = ui
					.add(ToolTab::new(tool_icons[index].1).selected(selected))
					.on_hover_text_at_pointer(tool.name());
				if response.clicked() {
					commands.insert_resource(NextState(Some(tool)))
				}
			}
		});
}

struct ToolTab {
	image: egui::Image,
	selected: bool,
}

impl ToolTab {
	fn new(texture_id: impl Into<egui::TextureId>) -> Self {
		Self {
			image: egui::Image::new(texture_id, [24.0, 24.0]),
			selected: false,
		}
	}

	pub fn selected(mut self, selected: bool) -> Self {
		self.selected = selected;
		self
	}
}

impl egui::Widget for ToolTab {
	fn ui(self, ui: &mut egui::Ui) -> egui::Response {
		let Self { image, selected } = self;

		let padding = egui::Vec2::splat(ui.spacing().button_padding.x);
		let size = image.size() + padding * 2.0;

		let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

		if ui.is_rect_visible(rect) {
			let (text, background) = match selected {
				// TODO: better color for bg
				true => (egui::Color32::WHITE, ui.visuals().widgets.hovered.bg_fill),
				false => {
					(
						match response.hovered() {
							true => egui::Color32::WHITE,
							// todo: better color for text
							false => ui.style().visuals.text_color(),
						},
						egui::Color32::TRANSPARENT,
					)
				}
			};

			ui.painter()
				.rect_filled(rect, egui::Rounding::none(), background);

			let image_rect = ui
				.layout()
				.align_size_within_rect(image.size(), rect.shrink2(padding));
			image.tint(text).paint_at(ui, image_rect)
		}

		response
	}
}
