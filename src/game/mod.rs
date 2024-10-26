use bevy::prelude::*;
use bullet::BulletPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

mod bullet;
mod movement;
mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((BulletPlugin, MovementPlugin, PlayerPlugin));
	}
}
