use super::{
	bullet::{BulletBundle, Collider, PlayerAttack, SelfDestruct},
	camera::CameraTarget,
	health::Destructable,
	movement::{Acceleration, Drag, Position, Velocity},
};
use bevy::{
	math::{bounding::Aabb2d, vec2, vec3},
	prelude::*,
};
use rand::random;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup)
			.add_systems(Update, apply_player_control);
	}
}

#[derive(Bundle)]
struct PlayerBundle {
	acceleration: Acceleration,
	velocity: Velocity,
	position: Position,
	destructable: Destructable,
	fire_rate: FireRate,
	drag: Drag,
	player_controlled: PlayerControlled,
	sprite_bundle: SpriteBundle,
}

#[derive(Component)]
pub struct PlayerControlled {
	speed: f32,
}

#[derive(Component)]
struct FireRate {
	timer: Timer,
}

fn setup(mut commands: Commands) {
	let position = vec2(0.0, 0.0);

	// Spawn player
	commands.spawn((
		PlayerBundle {
			fire_rate: FireRate {
				timer: Timer::from_seconds(0.1, TimerMode::Once),
			},
			acceleration: Acceleration {
				vector: vec2(0.0, 0.0),
			},
			velocity: Velocity {
				vector: vec2(0.0, 0.0),
			},
			destructable: Destructable { health: 100 },
			position: Position { vector: position },
			drag: Drag { value: 3.0 },
			player_controlled: PlayerControlled { speed: 1300.0 },
			sprite_bundle: SpriteBundle {
				transform: Transform {
					translation: vec3(position.x, position.y, 0.0),
					..default()
				},
				sprite: Sprite {
					color: Color::srgb(0.3, 0.3, 1.0),
					custom_size: Some(Vec2::new(40.0, 40.0)),
					..default()
				},
				..default()
			},
		},
		CameraTarget,
	));
}

fn apply_player_control(
	input: Res<ButtonInput<KeyCode>>,
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(
		&mut Acceleration,
		&mut FireRate,
		&Velocity,
		&Position,
		&PlayerControlled,
	)>,
) {
	for (mut accel, mut fire_rate, velocity, position, player_controlled) in query.iter_mut() {
		fire_rate.timer.tick(time.delta());

		if input.pressed(KeyCode::KeyA) {
			accel.vector.x = -player_controlled.speed;
		} else if input.pressed(KeyCode::KeyD) {
			accel.vector.x = player_controlled.speed;
		} else {
			accel.vector.x = 0.0;
		}

		if input.pressed(KeyCode::KeyW) {
			accel.vector.y = player_controlled.speed;
		} else if input.pressed(KeyCode::KeyS) {
			accel.vector.y = -player_controlled.speed;
		} else {
			accel.vector.y = 0.0;
		}

		if !fire_rate.timer.finished() {
			continue;
		}

		if input.pressed(KeyCode::ArrowRight) {
			fire_rate.timer.reset();
			let bullet_position = position.vector;

			let velocity_variable = (random::<f32>() * 200.0) - 100.0;
			let vertical_vector_variable = (random::<f32>() * 250.0) - 125.0;

			let transform = Transform {
				translation: vec3(bullet_position.x, bullet_position.y, 0.0),
				..default()
			};

			commands.spawn(BulletBundle {
				player_attack: PlayerAttack,
				self_destruct: SelfDestruct {
					timer: Timer::from_seconds(0.4, TimerMode::Once),
				},
				velocity: Velocity {
					vector: vec2(
						750.0 + velocity.vector.x + velocity_variable,
						vertical_vector_variable,
					),
				},
				position: Position {
					vector: bullet_position,
				},
				collider: Collider {
					aabb: Aabb2d::new(Vec2::new(-5.0, -5.0), Vec2::new(5.0, 5.0)),
				},
				sprite_bundle: SpriteBundle {
					transform,
					sprite: Sprite {
						color: Color::srgb(0.9, 0.3, 0.3),
						custom_size: Some(Vec2::new(10.0, 10.0)),
						..default()
					},
					..default()
				},
			});
		}
	}
}
