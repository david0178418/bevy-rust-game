use bevy::{prelude::*, window::WindowResized};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup).add_systems(
			Update,
			(
				update_camera_target,
				update_camera_position,
				on_window_resized,
			)
				.chain(),
		);
	}
}

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct NextCameraTarget;

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

fn update_camera_position(
	mut camera: Query<&mut Transform, (With<Camera2d>, Without<CameraTarget>)>,
	camera_target: Query<&Transform, With<CameraTarget>>,
	time: Res<Time>,
) {
	let Ok(mut camera) = camera.get_single_mut() else {
		return;
	};

	let Ok(camera_target) = camera_target.get_single() else {
		return;
	};

	let Vec3 { x, y, .. } = camera_target.translation;
	let direction = Vec3::new(x, y, camera.translation.z);

	// Applies a smooth effect to camera movement using interpolation between
	// the camera position and the player position on the x and y axes.
	// Here we use the in-game time, to get the elapsed time (in seconds)
	// since the previous update. This avoids jittery movement when tracking
	// the player.
	camera.translation = camera
		.translation
		.lerp(direction, time.delta_seconds() * 2.0);
}

fn update_camera_target(
	mut camera_target: Query<Entity, With<CameraTarget>>,
	mut next_camera_target: Query<Entity, With<NextCameraTarget>>,
	mut commands: Commands,
) {
	let Ok(camera_target) = camera_target.get_single_mut() else {
		return;
	};

	let Ok(next_camera_target) = next_camera_target.get_single_mut() else {
		return;
	};

	commands.entity(camera_target).remove::<CameraTarget>();
	commands
		.entity(next_camera_target)
		.remove::<NextCameraTarget>()
		.insert(CameraTarget);
}

fn on_window_resized(
	mut ev_window_resized: EventReader<WindowResized>,
	mut camera_transform_query: Query<&mut Transform, With<Camera2d>>,
) {
	for ev in ev_window_resized.read() {
		let scale_factor = 1920.0 / ev.width;
		let mut camera_transform = camera_transform_query.single_mut();

		camera_transform.scale = Vec3 {
			x: scale_factor,
			y: scale_factor,
			z: 1.0,
		};
	}
}
