use bevy::prelude::*;
use crate::movement::{Position, Velocity};

#[derive(Component)]
pub struct SelfDestruct {
	pub timer: Timer,
}

#[derive(Bundle)]
pub struct BulletBundle {
	pub velocity: Velocity,
	pub position: Position,
	pub self_destruct: SelfDestruct,
	pub sprite_bundle: SpriteBundle,
}

fn process_self_destruct(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut SelfDestruct)>,
) {
	for (entity, mut self_destruct) in query.iter_mut() {
		self_destruct.timer.tick(time.delta());

		if self_destruct.timer.finished() {
			commands.entity(entity).despawn();
		}
	}
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (
			process_self_destruct,
		));
	}
}
