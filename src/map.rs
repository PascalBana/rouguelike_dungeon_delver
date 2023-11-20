
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;
use crate::ascii::*;
use crate::gamestate::GameState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), spawn_map);
    }
}

const TILE_SIZE: f32 = 50.0;

#[derive(Component)]
pub struct TileCollider;

// add player exit tile to match statements
// also add spawn point for player
pub fn spawn_map(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
) {
    let file = File::open("assets/level_1.txt").expect("No file found");
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

// create genrate level function
// uses spawn map function and takes level as argument
// uses level to find what file to open
// and using match statements also decides whre to spawn player
// and enemies, as well as all other spawns

