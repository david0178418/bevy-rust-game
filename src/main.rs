mod game;

use bevy::{
	core::FrameCount,
	prelude::*,
	window::{EnabledButtons, PresentMode, WindowResolution, WindowTheme},
};
use game::GamePlugin;

const BASE_RESOLUTION_WIDTH: f32 = 1920.0;
const BASE_RESOLUTION_HEIGHT: f32 = 1080.0;
const INITIAL_SCREEN_RATIO: f32 = 15.0 / 8.0;

fn main() {
	println!(
		"format {} arguments",
		BASE_RESOLUTION_WIDTH * INITIAL_SCREEN_RATIO
	);
	App::new()
		.insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
		.add_plugins((
			DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					title: "Some Title".into(),
					resolution: WindowResolution::new(
						BASE_RESOLUTION_WIDTH / INITIAL_SCREEN_RATIO,
						BASE_RESOLUTION_HEIGHT / INITIAL_SCREEN_RATIO,
					),
					present_mode: PresentMode::AutoVsync,
					// Tells wasm to resize the window according to the available canvas
					// fit_canvas_to_parent: true,
					// Tells wasm not to override default event handling, like F5, Ctrl+R etc.
					prevent_default_event_handling: false,
					window_theme: Some(WindowTheme::Dark),
					resizable: true,
					enabled_buttons: EnabledButtons {
						maximize: false,
						..Default::default()
					},
					// This will spawn an invisible window
					// The window will be made visible in the make_visible() system after 3 frames.
					// This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
					visible: false,
					..default()
				}),
				..default()
			}),
			// LogDiagnosticsPlugin::default(),
			// FrameTimeDiagnosticsPlugin,
		))
		.add_plugins(bevy_dev_tools::DevToolsPlugin)
		.add_plugins(GamePlugin)
		.add_systems(Update, make_visible)
		.run();
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
	// The delay may be different for your app or system.
	if frames.0 == 3 {
		// At this point the gpu is ready to show the app so we can make the window visible.
		// Alternatively, you could toggle the visibility in Startup.
		// It will work, but it will have one white frame before it starts rendering
		window.single_mut().visible = true;
	}
}
