use bevy::{
	math::bounding::{Aabb2d, IntersectsVolume},
	prelude::*,
};

use super::{
	enemy::Enemy,
	movement::{Position, Velocity},
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (process_self_destruct, handle_collision));
	}
}

#[derive(Bundle)]
pub struct BulletBundle {
	pub player_attack: PlayerAttack,
	pub velocity: Velocity,
	pub collider: Collider,
	pub position: Position,
	pub self_destruct: SelfDestruct,
	pub sprite_bundle: SpriteBundle,
}

#[derive(Component)]
pub struct Collider {
	pub aabb: Aabb2d,
}

#[derive(Component)]
pub struct PlayerAttack;

#[derive(Component)]
pub struct SelfDestruct {
	pub timer: Timer,
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

fn handle_collision(
	mut commands: Commands,
	bullets_query: Query<(Entity, &Collider), With<PlayerAttack>>,
	enemy_query: Query<(Entity, &Collider), With<Enemy>>,
) {
	for (bullet_entity, bullet_collider) in bullets_query.iter() {
		for (enemy_entity, enemy_collider) in enemy_query.iter() {
			if bullet_collider.aabb.intersects(&enemy_collider.aabb) {
				commands.entity(bullet_entity).despawn();
				commands.entity(enemy_entity).despawn();
			}
		}
	}
}
