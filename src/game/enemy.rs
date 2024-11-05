use super::{
	bullet::Collider,
	health::Destructable,
	movement::{Position, Velocity},
};
use bevy::{math::bounding::Aabb2d, prelude::*};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, initialize_enemy);
	}
}

#[derive(Bundle)]
pub struct EnemyBundle {
	pub enemy: Enemy,
	pub velocity: Velocity,
	pub position: Position,
	pub destructable: Destructable,
	pub sprite_bundle: SpriteBundle,
	pub collider: Collider,
}

#[derive(Component)]
pub struct Enemy;

fn initialize_enemy(mut commands: Commands) {
	commands.spawn(EnemyBundle {
		enemy: Enemy,
		velocity: Velocity {
			vector: Vec2 { x: 10.0, y: -20.0 },
		},
		position: Position {
			vector: Vec2 { x: 200.0, y: 200.0 },
		},
		collider: Collider {
			aabb: Aabb2d {
				min: Vec2 { x: -20.0, y: -20.0 },
				max: Vec2 { x: 20.0, y: 20.0 },
			},
		},
		destructable: Destructable { health: 100 },
		sprite_bundle: SpriteBundle {
			transform: Transform {
				translation: Vec3 {
					x: 200.0,
					y: 200.0,
					z: 0.0,
				},
				..default()
			},
			sprite: Sprite {
				color: Color::srgb(1.0, 0.3, 1.0),
				custom_size: Some(Vec2::new(40.0, 40.0)),
				..default()
			},
			..default()
		},
	});
}
