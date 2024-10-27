mod bullet;
mod camera;
mod enemy;
mod movement;
mod player;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bullet::BulletPlugin;
use camera::{CameraPlugin, CameraTarget, NextCameraTarget};
use enemy::EnemyPlugin;
use movement::{MovementPlugin, Position};
use player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			BulletPlugin,
			MovementPlugin,
			EnemyPlugin,
			PlayerPlugin,
			CameraPlugin,
		))
		.add_systems(
			Update,
			handle_left_mouse_button.run_if(input_just_pressed(MouseButton::Left)),
		);
	}
}

fn handle_left_mouse_button(
	potential_targets: Query<Entity, (With<Position>, Without<CameraTarget>)>,
	mut commands: Commands,
) {
	let new_target = potential_targets.iter().next().unwrap();

	commands.entity(new_target).insert(NextCameraTarget);
}
