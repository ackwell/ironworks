use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, File, HtmlInputElement};

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
		.add_startup_system(startup_xivdir)
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

// web file picker testing
// ????
#[wasm_bindgen]
extern "C" {
	// TODO: name?
	#[wasm_bindgen(extends=File, js_name=File, typescript_type="File")]
	pub type File2;
	#[wasm_bindgen(structural, method, getter, js_class="File", js_name=webkitRelativePath)]
	pub fn webkit_relative_path(this: &File2) -> String;
}

// TODO: web stuff should be target gated
// TODO: error handling. should probably put in seperate fn from the main system and unwrap once
//       - looks like i can "chain" systems with In() and such to make an "error handling system"?
fn startup_xivdir() {
	// lets be real the websys shit here should probably be written in js and bound across instead of writing it in js but damn this is dumb
	let document = web_sys::window().unwrap().document().unwrap();

	let input = document
		.create_element("input")
		.unwrap()
		.dyn_into::<HtmlInputElement>()
		.unwrap();
	input.set_type("file");
	input.set_webkitdirectory(true);

	let closure = Closure::wrap(Box::new(move |event: Event| {
		info!("CHANGE");

		let files = event
			.target()
			.unwrap()
			.dyn_into::<HtmlInputElement>()
			.unwrap()
			.files()
			.unwrap();
		for i in 0..files.length() {
			let f2 = files.get(i).unwrap().dyn_into::<File2>().unwrap();
			let path = f2.webkit_relative_path();
			info!("path: {path:?}");
		}
	}) as Box<dyn FnMut(_)>);
	input
		.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
		.unwrap();
	closure.forget();

	// input.click doesn't work immediately as it needs user interaction - see if it's possible to hook up from in bevy?
	document.body().unwrap().append_child(&input).unwrap();

	info!("ran!");
}
