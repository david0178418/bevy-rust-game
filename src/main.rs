mod movement;
mod bullet;
mod player;

use bevy::{
	prelude::*,
	core::FrameCount,
	window::{
		PresentMode,
		WindowTheme,
	}
};
use movement::MovementPlugin;
use bullet::BulletPlugin;
use player::PlayerPlugin;

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
		.add_plugins((
			DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					title: "Some Title".into(),
					resolution: (1024., 728.).into(),
					present_mode: PresentMode::AutoVsync,
					// Tells wasm to resize the window according to the available canvas
					fit_canvas_to_parent: true,
					// Tells wasm not to override default event handling, like F5, Ctrl+R etc.
					prevent_default_event_handling: false,
					window_theme: Some(WindowTheme::Dark),
					enabled_buttons: bevy::window::EnabledButtons {
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
		.add_plugins((
			MovementPlugin,
			BulletPlugin,
			PlayerPlugin
		))
		.add_systems(Startup, setup)
		.add_systems(Update, (
			bevy::window::close_on_esc,
			make_visible,
		))
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

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}
