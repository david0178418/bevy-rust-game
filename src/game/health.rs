use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, destroy_system);
	}
}

#[derive(Component)]
pub struct Destructable {
	pub health: i32,
}

fn destroy_system(mut commands: Commands, mut query: Query<(Entity, &mut Destructable)>) {
	query
		.iter_mut()
		.filter(|(_, destructable)| destructable.health <= 0)
		.map(|(e, _)| e)
		.for_each(|e| commands.entity(e).despawn());
}
