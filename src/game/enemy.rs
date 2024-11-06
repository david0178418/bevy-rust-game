use super::{
	bullet::Collider,
	health::Destructable,
	movement::{Position, Velocity},
};
use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::{thread_rng, Rng};

#[derive(Resource)]
pub struct EnemySpawner {
	pub timer: Timer,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, initialize_enemy)
			.add_systems(Update, enemy_spawner);
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
	let position_vector = Vec2 {
		x: thread_rng().gen_range(-200.0..200.0),
		y: thread_rng().gen_range(-200.0..200.0),
	};
	commands.spawn(EnemyBundle {
		enemy: Enemy,
		velocity: Velocity {
			vector: Vec2 {
				x: thread_rng().gen_range(-20.0..20.0),
				y: thread_rng().gen_range(-20.0..20.0),
			},
		},
		position: Position {
			vector: position_vector,
		},
		collider: Collider {
			aabb: Aabb2d {
				min: Vec2 { x: 0.0, y: 0.0 },
				max: Vec2 { x: 0.0, y: 0.0 },
			},
		},
		destructable: Destructable { health: 100 },
		sprite_bundle: SpriteBundle {
			transform: Transform {
				translation: Vec3 {
					// use position_vector for x and y
					x: position_vector.x,
					y: position_vector.y,
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

fn enemy_spawner(commands: Commands, time: Res<Time>, mut enemy_spawner: ResMut<EnemySpawner>) {
	if enemy_spawner.timer.tick(time.delta()).just_finished() {
		initialize_enemy(commands);
	}
}
