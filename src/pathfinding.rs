use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{player::{PLAYER_SIZE, 
    Player, wall_collision_check}, 
    map::TileCollider,
    gamestate::GameState,
};

pub struct PathfinderPlugin;

impl Plugin for PathfinderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                FixedUpdate, (
                move_towards_player,
            ).run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct Pathinder {
    pub vision: f32,
    pub speed: f32,
}

pub fn find_player_location(
    target_player_position: Vec3,
    pathfinder_position: Vec3,
    pathfinder_vision: f32,
) -> bool { 
        let collision = collide(
            target_player_position,
            Vec2::splat(PLAYER_SIZE),
            pathfinder_position,
            Vec2::splat(pathfinder_vision),
        );

        if collision.is_some() {
            return true;
        } 
    false
}

pub fn move_towards_player(
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>, Without<Pathinder>)>,
    player_transform_query: Query<&Transform, With<Player>>,
    mut pathfinder_query: Query<(&mut Transform, &Pathinder), Without<Player>>,
    time: Res<Time>,
    
) {
    let player_transform = player_transform_query.single();
    for (mut pathfinder_transform, pathfinder) in pathfinder_query.iter_mut() {
        if find_player_location(
            player_transform.translation, 
            pathfinder_transform.translation, 
            pathfinder.vision
        ) {
            let mut x_delta = 0.0;
            if pathfinder_transform.translation.x < player_transform.translation.x {
                x_delta += pathfinder.speed * time.delta_seconds();
            }
            if pathfinder_transform.translation.x > player_transform.translation.x {
                x_delta -= pathfinder.speed * time.delta_seconds();
            }

            let mut y_delta = 0.0;
            if pathfinder_transform.translation.y < player_transform.translation.y {
                y_delta += pathfinder.speed * time.delta_seconds();
            }
            if pathfinder_transform.translation.y > player_transform.translation.y {
                y_delta -= pathfinder.speed * time.delta_seconds();
            }

            let target_pathfinder_position = pathfinder_transform.translation + Vec3::new(x_delta, 0.0, 0.0);
            if wall_collision_check(target_pathfinder_position, &wall_query) {
                pathfinder_transform.translation = target_pathfinder_position;
            }

            let target_pathfinder_position = pathfinder_transform.translation + Vec3::new(0.0, y_delta, 0.0);
            if wall_collision_check(target_pathfinder_position, &wall_query) {
                pathfinder_transform.translation = target_pathfinder_position;
            }
        }
    }
}