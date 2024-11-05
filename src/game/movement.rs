use bevy::prelude::*;

use super::bullet::Collider;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			FixedUpdate,
			(
				apply_acceleration,
				apply_velocity,
				apply_drag,
				update_moving_sprites,
				update_moving_colliders,
			)
				.chain(),
		);
	}
}

#[derive(Component)]
pub struct Acceleration {
	pub vector: Vec2,
}

#[derive(Component)]
pub struct Velocity {
	pub vector: Vec2,
}

#[derive(Component)]
pub struct Position {
	pub vector: Vec2,
}

#[derive(Component)]
pub struct Drag {
	pub value: f32,
}

fn apply_acceleration(time: Res<Time>, mut query: Query<(&mut Velocity, &Acceleration)>) {
	for (mut vel, acc) in query.iter_mut() {
		vel.vector += acc.vector * time.delta_seconds();
	}
}

fn apply_drag(time: Res<Time>, mut query: Query<(&mut Velocity, &Drag)>) {
	for (mut vel, drag) in query.iter_mut() {
		// apply drag to vector such that it normalizes to 0
		vel.vector *= 1.0 - (drag.value * time.delta_seconds());
	}
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Position, &Velocity)>) {
	for (mut position, velocity) in query.iter_mut() {
		position.vector.x += time.delta_seconds() * velocity.vector.x;
		position.vector.y += time.delta_seconds() * velocity.vector.y;
	}
}

fn update_moving_sprites(mut query: Query<(&mut Transform, &Position), With<Velocity>>) {
	for (mut transform, position) in query.iter_mut() {
		transform.translation.x = position.vector.x;
		transform.translation.y = position.vector.y;
	}
}

fn update_moving_colliders(mut query: Query<(&mut Collider, &Position, &Sprite), With<Velocity>>) {
	for (mut collider, position, sprite) in query.iter_mut() {
		let half_width = sprite.custom_size.unwrap().x / 2.0;
		let half_height = sprite.custom_size.unwrap().x / 2.0;

		collider.aabb.min.x = position.vector.x - half_width;
		collider.aabb.min.y = position.vector.y - half_height;
		collider.aabb.max.x = position.vector.x + half_width;
		collider.aabb.max.y = position.vector.y + half_height;
	}
}
