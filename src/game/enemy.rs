use super::movement::{Position, Velocity};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, initialize_enemy);
	}
}

#[derive(Bundle)]
pub struct EnemyBundle {
	pub velocity: Velocity,
	pub position: Position,
	pub sprite_bundle: SpriteBundle,
}

fn initialize_enemy(mut commands: Commands) {
	commands.spawn(EnemyBundle {
		velocity: Velocity {
			vector: Vec2 { x: 10.0, y: -20.0 },
		},
		position: Position {
			vector: Vec2 { x: 200.0, y: 200.0 },
		},
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
