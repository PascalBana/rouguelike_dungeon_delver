use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::player::player::{Player, PLAYER_SIZE};
use crate::enemy::{Enemy, ENEMY_SIZE};

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

pub fn check_enemy_collision(
    player_transform: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Health), (With<Enemy>, Without<Player>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let player_transform = player_transform.single();
    for (enemy_transform, mut enemy_health) in enemy_query.iter_mut() {
            let collision = collide(
                player_transform.translation,
                Vec2::splat(PLAYER_SIZE * 3.0),
                enemy_transform.translation,
                Vec2::splat(ENEMY_SIZE),
            );

            if collision.is_some() && keyboard_input.just_pressed(KeyCode::Space){
                enemy_health.health -= 1;
                println!("Enemy Health: {}", enemy_health.health);
            }
        }
}
