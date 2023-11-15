use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub health: i32,
}

pub fn health_check(
    mut commands: Commands,
    health_query: Query<(Entity, &Health)>,
) {
    for (entity, health) in health_query.iter() {
        if health.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}


