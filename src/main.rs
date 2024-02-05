use bevy::{
	prelude::*,
	core::FrameCount,
	math::{
		vec2,
		vec3,
	},
	window::{
		PresentMode,
		WindowTheme,
	}
};
use rand::random;

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
		.add_systems(Startup, setup)
		.add_systems(Update, (
			bevy::window::close_on_esc,
			make_visible,
			apply_player_control,
			process_self_destruct_on_timer,
		))
		.add_systems(FixedUpdate, (
			process_fire_rate,
			apply_acceleration,
			apply_velocity,
			apply_drag,
			update_moving_sprites,
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

#[derive(Component)]
struct PlayerControlled {
	speed: f32,
}

#[derive(Component)]
struct Acceleration  {
	vector: Vec2,
}

#[derive(Component)]
struct Velocity  {
	vector: Vec2,
}

#[derive(Component)]
struct Position  {
	vector: Vec2,
}

#[derive(Component)]
struct Drag {
	value: f32,
}

#[derive(Component)]
struct FireRate {
	remaining_ms: f32,
	ms_delay: f32,
}

#[derive(Component)]
struct SelfDestruct {
	timer: Timer,
}

#[derive(Bundle)]
struct PlayerBundle {
	acceleration: Acceleration,
	velocity: Velocity,
	position: Position,
	fire_rate: FireRate,
	drag: Drag,
	player_controlled: PlayerControlled,
	sprite_bundle: SpriteBundle,
}

#[derive(Bundle)]
struct BulletBundle {
	velocity: Velocity,
	position: Position,
	self_destruct: SelfDestruct,
	sprite_bundle: SpriteBundle,
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());

	let position = vec2(0.0, 0.0);
	// Spawn player
	commands.spawn(
		PlayerBundle {
			acceleration: Acceleration {
				vector: vec2(0.0, 0.0),
			},
			velocity: Velocity {
				vector: vec2(0.0, 0.0),
			},
			position: Position {
				vector: position,
			},
			drag: Drag {
				value: 3.0,
			},
			fire_rate: FireRate {
				remaining_ms: 0.0,
				ms_delay: 100.0,
			},
			player_controlled: PlayerControlled {
				speed: 1300.0,
			},
			sprite_bundle: SpriteBundle {
				transform: Transform {
					translation: vec3(position.x, position.y, 0.0),
					..default()
				},
				sprite: Sprite {
					color: Color::rgb(0.3, 0.3, 0.9),
					custom_size: Some(Vec2::new(40.0, 40.0)),
					..default()
				},
				..default()
			},
		},
	);
}

fn apply_player_control(
	input: Res<Input<KeyCode>>,
	mut commands: Commands,
	mut query: Query<(
		&mut Acceleration,
		&mut FireRate,
		&Velocity,
		&Position,
		&PlayerControlled,
	)>,
) {
	for (
		mut accel,
		mut fire_rate,
		velocity,
		position,
		player_controlled
	) in query.iter_mut() {
		if input.pressed(KeyCode::A) {
			accel.vector.x = -player_controlled.speed;
		} else if input.pressed(KeyCode::D) {
			accel.vector.x = player_controlled.speed;
		} else {
			accel.vector.x = 0.0;
		}

		if input.pressed(KeyCode::W) {
			accel.vector.y = player_controlled.speed;
		} else if input.pressed(KeyCode::S) {
			accel.vector.y = -player_controlled.speed;
		} else {
			accel.vector.y = 0.0;
		}

		if fire_rate.remaining_ms > 0.0 {
			return;
		}

		if input.pressed(KeyCode::Right) {
			fire_rate.remaining_ms = fire_rate.ms_delay;
			let bullet_position = position.vector;

			let velocity_variable = (random::<f32>() * 200.0) - 100.0;
			let vertical_vector_variable = (random::<f32>() * 250.0) - 125.0;

			commands.spawn(
				BulletBundle {
					self_destruct: SelfDestruct {
						timer: Timer::from_seconds(0.4, TimerMode::Once),
					},
					velocity: Velocity {
						vector: vec2(
							750.0 + velocity.vector.x + velocity_variable,
							vertical_vector_variable
						),
					},
					position: Position {
						vector: bullet_position,
					},
					sprite_bundle: SpriteBundle {
						transform: Transform {
							translation: vec3(bullet_position.x, bullet_position.y, 0.0),
							..default()
						},
						sprite: Sprite {
							color: Color::rgb(0.9, 0.3, 0.3),
							custom_size: Some(Vec2::new(10.0, 10.0)),
							..default()
						},
						..default()
					},
				}
			);
		}
	}
}

fn apply_acceleration(
	time: Res<Time>,
	mut query: Query<(
		&mut Velocity,
		&Acceleration,
	)>,
) {
	for (mut vel, acc) in query.iter_mut() {
		vel.vector += acc.vector * time.delta_seconds();
	}
}

fn apply_drag(
	time: Res<Time>,
	mut query: Query<(
		&mut Velocity,
		&Drag,
	)>,
) {
	for (mut vel, drag) in query.iter_mut() {
		// apply drag to vector such that it normalizes to 0
		vel.vector *= 1.0 - (drag.value * time.delta_seconds());
	}
}

fn apply_velocity(
	time: Res<Time>,
	mut query: Query<(
		&mut Position,
		&Velocity,
	)>,
) {
	for (mut position, velocity) in query.iter_mut() {
		position.vector.x += time.delta_seconds() * velocity.vector.x;
		position.vector.y += time.delta_seconds() * velocity.vector.y;
	}
}

fn update_moving_sprites (
	mut query: Query<
		(
			&mut Transform,
			&Position,
		),
		With<Velocity>
	>,
) {
	for (mut transform, position) in query.iter_mut() {
		transform.translation.x = position.vector.x;
		transform.translation.y = position.vector.y;
	}
}

fn process_fire_rate(
	time: Res<Time>,
	mut query: Query<&mut FireRate>,
) {
	for mut fire_rate in &mut query {
		fire_rate.remaining_ms -= time.delta_seconds() * 1000.0;
	}
}

fn process_self_destruct_on_timer(
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

