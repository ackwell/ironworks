use bevy::{
	input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
	prelude::*,
};
use smooth_bevy_cameras::{
	controllers::orbit::{
		ControlEvent, OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin,
	},
	LookTransformPlugin,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(LookTransformPlugin)
			.add_plugin(OrbitCameraPlugin::new(true))
			.add_startup_system(spawn_camera)
			.add_system(camera_controls);
	}
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn_bundle(OrbitCameraBundle::new(
		OrbitCameraController {
			// Smoothing the camera does _not_ mesh with desktop update tick.
			smoothing_weight: 0.0,
			..Default::default()
		},
		PerspectiveCameraBundle::default(),
		Vec3::new(2.0, 0.0, 8.0),
		Vec3::ZERO,
	));
}

// Slightly tweaked copy of the default controls from the library because I didn't like the control scheme.
fn camera_controls(
	mut control_events: EventWriter<ControlEvent>,
	mut mouse_wheel_reader: EventReader<MouseWheel>,

	mut last_cursor_position: Local<Vec2>,
	mut cursor_moved: EventReader<CursorMoved>,

	mouse_buttons: Res<Input<MouseButton>>,
	controllers: Query<&OrbitCameraController>,
) {
	// Get the controller for the camera.
	let controller = match controllers.iter().find(|controller| controller.enabled) {
		Some(controller) => controller,
		_ => return,
	};

	let OrbitCameraController {
		mouse_rotate_sensitivity,
		mouse_translate_sensitivity,
		mouse_wheel_zoom_sensitivity,
		pixels_per_line,
		..
	} = *controller;

	// Build the full mouse movement delta.
	// Avoiding MouseMotion for this, as it reports bad values over a WSL/X11 setup.
	let new_pos = cursor_moved
		.iter()
		.next_back()
		.map(|event| event.position * Vec2::new(1.0, -1.0))
		.unwrap_or(*last_cursor_position);

	let cursor_delta = new_pos - *last_cursor_position;
	*last_cursor_position = new_pos;

	// LMB translates on current plane.
	if mouse_buttons.pressed(MouseButton::Left) {
		control_events.send(ControlEvent::TranslateTarget(
			mouse_translate_sensitivity * cursor_delta,
		));
	}

	// RMB orbits current target.
	if mouse_buttons.pressed(MouseButton::Right) {
		control_events.send(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
	}

	// Mouse wheel zooms current target.
	let zoom = mouse_wheel_reader.iter().fold(1.0, |total, event| {
		let amount = match event.unit {
			MouseScrollUnit::Line => event.y,
			MouseScrollUnit::Pixel => event.y * pixels_per_line,
		};
		total * (1.0 - amount * mouse_wheel_zoom_sensitivity)
	});
	control_events.send(ControlEvent::Zoom(zoom));
}
