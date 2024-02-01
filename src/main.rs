use bevy::{
	core::FrameCount,
    prelude::*,
	math::{
		vec2,
		vec3,
	},
    window::{
		PresentMode,
		WindowTheme
	},
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        // .add_plugins(DefaultPlugins)
		.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Some Title".into(),
                    resolution: (1024., 728.).into(),
                    present_mode: PresentMode::AutoNoVsync,
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
		.add_systems(Update, (
			bevy::window::close_on_esc,
			make_visible,
			apply_player_control,
		))
		.add_systems(Startup, setup)
		.add_systems(FixedUpdate, (
			apply_acceleration,
			apply_velocity,
			apply_drag,
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
struct Drag {
	value: f32,
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());

	// Spawn player
	commands.spawn((
		SpriteBundle {
			transform: Transform {
				translation: vec3(0.0, 0.0, 0.0),
				..default()
			},
			sprite: Sprite {
				color: Color::rgb(0.3, 0.3, 0.9),
				custom_size: Some(Vec2::new(40.0, 40.0)),
				..default()
			},
			..default()
		},
		Acceleration {
			vector: vec2(0.0, 0.0),
		},
		Velocity {
			vector: vec2(0.0, 0.0),
		},
		Drag {
			value: 3.0,
		},
		PlayerControlled {
			speed: 1300.0,
		},
	));
}

fn apply_player_control(
	input: Res<Input<KeyCode>>,
	_: Res<Time>,
	mut query: Query<(
		&mut Acceleration,
		&PlayerControlled
	)>,
) {
	for (mut accel, player_controlled) in query.iter_mut() {
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
	}
}

fn apply_acceleration(
	_: Res<Input<KeyCode>>,
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
	_: Res<Input<KeyCode>>,
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
	_: Res<Input<KeyCode>>,
	time: Res<Time>,
	mut query: Query<(
		&mut Transform,
		&Velocity,
	)>,
) {
	for (mut transform, velocity) in query.iter_mut() {
		transform.translation.x += time.delta_seconds() * velocity.vector.x;
		transform.translation.y += time.delta_seconds() * velocity.vector.y;
	}
}

