use bevy::transform::components::Transform;

use crate::enemy::{ENEMY_SIZE, Enemy};
use crate::health::Health;

use crate::ascii::*;
use crate::map::TileCollider;
use crate::pathfinding::Pathinder;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub const PLAYER_SIZE: f32 = 50.0;
const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player {
    player_speed: f32,
}

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>, Without<Pathinder>)>,
    mut player_query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let (mut transform, player) = player_query.single_mut(); 
    
    let mut y_delta = 0.0;
    if keyboard_input.pressed(KeyCode::W) {
        y_delta +=  player.player_speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::S) {
        y_delta -= player.player_speed * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        x_delta -= player.player_speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::D) {
        x_delta += player.player_speed * time.delta_seconds();
    }

    let target_player_position = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target_player_position, &wall_query) {
        transform.translation = target_player_position;
    }

    let target_player_position = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target_player_position, &wall_query) {
        transform.translation = target_player_position;
    }
}

pub fn wall_collision_check(
    target_player_position: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>, Without<Pathinder>)>
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_position, 
            Vec2::splat(PLAYER_SIZE * 0.9),
            wall_transform.translation, 
            Vec2::splat(PLAYER_SIZE)
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

pub fn spawn_player(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
) {
        let mut background_sprite = TextureAtlasSprite::new(0);
        background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
        background_sprite.custom_size = Some(Vec2::splat(PLAYER_SIZE));  

        let mut sprite = TextureAtlasSprite::new(1);
        sprite.color = Color::rgb(0.0, 0.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(PLAYER_SIZE));

        commands.spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform::from_translation(Vec3::new(60.0, -50.0, 900.0)),
            ..default()
        })
        .insert(Player {
            player_speed: PLAYER_SPEED,
        })
        .insert(Name::new("Player"))
        .with_children(|parent| {
            parent
            .spawn(SpriteSheetBundle {
                sprite: background_sprite,
                texture_atlas: ascii.0.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
                ..default()
            })
            .insert(Name::new("Background"));
        });
}


pub fn attack_enemy(
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
