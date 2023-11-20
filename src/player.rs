use bevy::transform::components::Transform;

use crate::enemy::{ENEMY_SIZE, Enemy};
use crate::health::Health;
use crate::gamestate::GameState;    

use crate::ascii::*;
use crate::map::TileCollider;
use crate::pathfinding::Pathinder;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Game), (
                spawn_player, 
            ))
            .add_systems(
            FixedUpdate, (
                player_movement, 
                camera_follow, 
                attack_enemy, 
            ).run_if(in_state(GameState::Game)));
    }
    
}

pub const PLAYER_SIZE: f32 = 50.0;
const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player {
    speed: f32,
    direction: u16,
    timer: Timer,
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
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    let (mut transform, mut player) = player_query.single_mut(); 
    
    let mut y_delta = 0.0;
    if keyboard_input.pressed(KeyCode::W) {
        y_delta +=  player.speed * time.delta_seconds();
        player.direction = 360;
    }
    if keyboard_input.pressed(KeyCode::S) {
        y_delta -= player.speed * time.delta_seconds();
        player.direction = 180;
    }

    let mut x_delta = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        x_delta -= player.speed * time.delta_seconds();
        player.direction = 270;
    }
    if keyboard_input.pressed(KeyCode::D) {
        x_delta += player.speed * time.delta_seconds();
        player.direction = 90;
    }

    let target_player_position = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target_player_position, &wall_query) {
        transform.translation = target_player_position;
    }

    let target_player_position = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target_player_position, &wall_query) {
        transform.translation = target_player_position;
    }
    // cause player movement to change player direction

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
            speed: PLAYER_SPEED,
            direction: 270,
            timer: Timer::from_seconds(3.0, TimerMode::Once)
        })
        .insert(Name::new("Player"))
        .insert(Health {
            health: 100,
        })
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
    mut player_transform: Query<(&Transform, &mut Player)>,
    mut enemy_query: Query<(&mut Transform, &mut Health), (With<Enemy>, Without<Player>, Without<TileCollider>)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>, Without<Pathinder>)>,
) {
    let (player_transform, mut player) = player_transform.single_mut();
    player.timer.tick(time.delta());

    for (enemy_transform, mut enemy_health) in enemy_query.iter_mut() {
            let collision = collide(
                player_transform.translation,
                Vec2::splat(PLAYER_SIZE * 3.0),
                enemy_transform.translation,
                Vec2::splat(ENEMY_SIZE),
            );

            // add attack timer to create cooldown effect for attacks
            if collision.is_some() && 
            keyboard_input.just_pressed(KeyCode::Space) && 
            player.timer.finished()
            {
                enemy_health.health -= 1;
                println!("Enemy Health: {}", enemy_health.health);
                player.timer.reset();
                // add knock back effect based on player direction
                knock_back(player.direction, &wall_query, enemy_transform);
                
            }
        }
}

// knock back funstion which knocks back enemy based on player direction
pub fn knock_back(
    player_direction: u16,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>, Without<Pathinder>)>,
    mut enemy_transform: Mut<'_, Transform>,
) {
        enemy_transform.translation = 
        knock_back_measurement(
            enemy_transform.translation, 
            wall_query, 
            player_direction
        ) + 
        enemy_transform.translation;
}

pub fn knock_back_measurement(
    enemy_position: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>, Without<Pathinder>)>, 
    player_direction: u16
) -> Vec3 {
    let knock_back_prediction = match player_direction {
        360 => Vec3::new(0.0, 100.0, 0.0),
        90 => Vec3::new(100.0, 0.0, 0.0),
        180 => Vec3::new(0.0, -100.0, 0.0),
        270 => Vec3::new(-100.0, 0.0, 0.0),
        _ => Vec3::new(0.0, 0.0, 0.0),
    }; 
    // find all wall that are within  50 pixels perpendicular of the player direction
    // if there is a wall within 50 pixels perpendicular of the player direction
    // create an array of all the walls within 50 pixels perpendicular of the player direction
    // then find which wall is closest to the player and return the distance between the player and the wall
    let mut perpendicular_walls: Vec<Vec3> = Vec::new();
    for wall in wall_query.iter() {
        match player_direction {
            360 | 180 => {
                if enemy_position.x + 45.0 > wall.translation.x && enemy_position.x - 45.0 < wall.translation.x {
                    perpendicular_walls.push(wall.translation);
                }
            }
            90 | 270 => {
                if enemy_position.y + 45.0 > wall.translation.y && enemy_position.y - 45.0 < wall.translation.y {
                    perpendicular_walls.push(wall.translation);
                }
            }
            _ => {}
        }
    }
    let mut collision_wall = Vec3::new(0.0, 0.0, 0.0);
    match player_direction {
        360 => {
            let mut walls_above_enemy: Vec<&Vec3> = Vec::new();
            for wall in perpendicular_walls.iter() {
                if wall.y > enemy_position.y {
                    walls_above_enemy.push(wall);
                }
            }
            walls_above_enemy.sort_by(|a, b| {
                a.x.partial_cmp(&b.x).unwrap()
            });
            collision_wall = **walls_above_enemy.last().unwrap();
        }
        270 => {
            let mut walls_left_of_enemy: Vec<&Vec3> = Vec::new();
            for wall in perpendicular_walls.iter() {
                if wall.x < enemy_position.x {
                    walls_left_of_enemy.push(wall);
                }
            }
            walls_left_of_enemy.sort_by(|a, b| {
                a.y.partial_cmp(&b.y).unwrap()
            });
            collision_wall = **walls_left_of_enemy.last().unwrap();
        }
        180 => {
            let mut walls_below_enemy: Vec<&Vec3> = Vec::new();
            for wall in perpendicular_walls.iter() {
                if wall.y < enemy_position.y {
                    walls_below_enemy.push(wall);
                }
            }
            walls_below_enemy.sort_by(|a, b| {
                a.x.partial_cmp(&b.x).unwrap()
            });
            collision_wall = walls_below_enemy[0].clone();
        }
        90 => {
            let mut walls_right_of_enemy: Vec<&Vec3> = Vec::new();
            for wall in perpendicular_walls.iter() {
                if wall.x > enemy_position.x {
                    walls_right_of_enemy.push(wall);
                }
            }
            walls_right_of_enemy.sort_by(|a, b| {
                a.y.partial_cmp(&b.y).unwrap()
            });
            dbg!(&walls_right_of_enemy);
            collision_wall = walls_right_of_enemy[0].clone();
        }
        _ => ()
    };
    dbg!(collision_wall);
    match player_direction {
        360 => {
            if knock_back_prediction.y + enemy_position.y + 50.0 > collision_wall.y {
                return Vec3::new(0.0, collision_wall.y - enemy_position.y - 50.0, 0.0);
            } else {
                return knock_back_prediction;
            }
        }
        90 => {
            if knock_back_prediction.x + enemy_position.x + 50.0 > collision_wall.x {
                return Vec3::new(collision_wall.x - enemy_position.x - 50.0, 0.0, 0.0);
            }else {
                dbg!(knock_back_prediction);
                return knock_back_prediction;
            }
        }
        180 => {
            if knock_back_prediction.y + enemy_position.y - 50.0 < collision_wall.y {
                return Vec3::new(0.0, collision_wall.y - enemy_position.y + 50.0, 0.0);
            }else {
                return knock_back_prediction;
            }
        }
        270 => {
            if knock_back_prediction.x + enemy_position.x - 50.0 < collision_wall.x {
                return Vec3::new(collision_wall.x - enemy_position.x + 50.0, 0.0, 0.0);
            } else {
                return knock_back_prediction;
            }
        }
        _ => Vec3::new(0.0, 0.0, 0.0)
    }
}
