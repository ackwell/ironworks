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

// web file picker testing
// TODO: web stuff should be target gated
// ????
#[wasm_bindgen]
extern "C" {
	// TODO: name?
	#[wasm_bindgen(extends=File, js_name=File, typescript_type="File")]
	pub type File2;
	#[wasm_bindgen(structural, method, getter, js_class="File", js_name=webkitRelativePath)]
	pub fn webkit_relative_path(this: &File2) -> String;
}

// TODO: name
struct EventRequestSqpack;

// TODO: error handling. should probably put in seperate fn from the main system and unwrap once
//       - looks like i can "chain" systems with In() and such to make an "error handling system"?
fn system_request_sqpack(mut events: EventReader<EventRequestSqpack>) {
	for _event in events.iter() {
		request_directory().unwrap();
	}
}

// TODO: name
fn request_directory() -> Result<(), JsValue> {
	let document = web_sys::window().unwrap().document().unwrap();
	let input = document
		.create_element("input")?
		.dyn_into::<HtmlInputElement>()?;
	input.set_type("file");
	input.set_webkitdirectory(true);

	// TODO: avoid leaking one of these every time
	let closure = Closure::wrap(Box::new(on_file_change) as Box<dyn FnMut(_)>);
	input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
	closure.forget();

	input.click();

	Ok(())
}

// TODO: error handling here - few unwraps here are entirely possible
fn on_file_change(event: Event) {
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
}
