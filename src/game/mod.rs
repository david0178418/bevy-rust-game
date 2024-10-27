mod bullet;
mod enemy;
mod movement;
mod player;

use bevy::prelude::*;
use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((BulletPlugin, MovementPlugin, EnemyPlugin, PlayerPlugin));
	}
}
