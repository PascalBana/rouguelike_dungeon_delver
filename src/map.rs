
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::{prelude::*, sprite::collide_aabb::collide};
use crate::{ascii::*, player::{spawn_player, Player}, enemy::spawn_enemy};
use crate::gamestate::GameState;


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Game(GameLevel::Level1)), 
                load_level
            )
            .add_systems(
                FixedUpdate,
                exit_level.run_if(in_state(GameState::Game(GameLevel::Level1))),
            );
    }
}

const TILE_SIZE: f32 = 50.0;


#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct ExitTile;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Resource)]
pub enum GameLevel {
    #[default]
    Level1,
    Level2,
    Level3,
}

pub fn load_level(
    level: Res<State<GameState>>,
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
) {
    match level.get() {
        GameState::Game(GameLevel::Level1) => {
            let file = File::open("assets/level_1.txt").expect("No file found");
            let player_spawn_point = Vec3::new(450.0, -250.0, 890.0);
            let enemy_spawn_point = Vec3::new(600.0, -50.0, 880.0);
            spawn_map(&mut commands, &ascii, file);
            spawn_player(&mut commands, &ascii, player_spawn_point);
            spawn_enemy(&mut commands, &ascii, enemy_spawn_point);
        }
        GameState::Game(GameLevel::Level2) => {
            let file = File::open("assets/level_2.txt").expect("No file found");
        }
        GameState::Game(GameLevel::Level3) => {
            let file = File::open("assets/level_3.txt").expect("No file found");
        }
        _ => {}
    }
}

pub fn spawn_map(
    commands: &mut Commands,
    ascii: &Res<AsciiSheet>,
    file: File,
) {
    let mut tiles= Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = match char {
                    '#' => {
                        let mut sprite = TextureAtlasSprite::new(char as usize);
                        sprite.color = Color::rgb(1.0, 1.0, 1.0);
                        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

                        commands.spawn(SpriteSheetBundle {
                            sprite,
                            texture_atlas: ascii.0.clone(),
                            transform: Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE, -(y as f32 )* TILE_SIZE, 100.0)),
                            ..default()
                        })
                        .insert(TileCollider)
                        .id()
                    }
                    '%' => {
                        let mut sprite = TextureAtlasSprite::new(206);
                        sprite.color = Color::rgb(1.0, 1.0, 1.0);
                        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

                        commands.spawn(SpriteSheetBundle {
                            sprite,
                            texture_atlas: ascii.0.clone(),
                            transform: Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE, -(y as f32 )* TILE_SIZE, 100.0)),
                            ..default()
                        })
                        .insert(ExitTile)
                        .id()
                    }
                    _ => {
                        let mut sprite = TextureAtlasSprite::new(176);
                        sprite.color = Color::rgb(0.5, 0.5, 0.5);
                        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

                        commands.spawn(SpriteSheetBundle {
                            sprite,
                            texture_atlas: ascii.0.clone(),
                            transform: Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE, -(y as f32 )* TILE_SIZE, 100.0)),
                            ..default()
                        }).id()
                    }
                };
                tiles.push(tile);
            }
        }
    }
}

fn exit_level(
    player_transform: Query<&Transform, With<Player>>,
    exit_tile_transform: Query<&Transform, With<ExitTile>>,
    mut gamestate: ResMut<NextState<GameState>>,
) {
    let player_transform = player_transform.single();
    let exit_tile_transform = exit_tile_transform.single();
    if player_reached_exit_tile(
        player_transform.translation, 
        exit_tile_transform.translation) {
            panic!("Player reached exit tile");
            gamestate.set(GameState::Game(GameLevel::Level2));
        }
}

fn player_reached_exit_tile (
    player_position: Vec3,
    exit_tile_position: Vec3,
) -> bool {
    let collision = collide(
        player_position,
        Vec2::splat(TILE_SIZE),
        exit_tile_position,
        Vec2::splat(TILE_SIZE),
    );

    if collision.is_some() {
        return true;
    }
    false
}

fn despawn_level(
    mut commands: Commands,
    mut query: Query<Entity, Without<Camera2d>>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}