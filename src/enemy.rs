use bevy::prelude::*;
use crate::{ascii::ascii::AsciiSheet, health::Health, pathfinding::Pathinder};

pub const ENEMY_SIZE: f32 = 50.0;
const ENEMY_SPEED: f32 = 10.0;

#[derive(Component)]
pub struct Enemy;

pub fn spawn_enemy(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
) {
    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.2, 0.2, 0.2);
    background_sprite.custom_size = Some(Vec2::splat(ENEMY_SIZE));  

    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(1.0, 0.1, 0.1);
    sprite.custom_size = Some(Vec2::splat(ENEMY_SIZE));

    commands.spawn(SpriteSheetBundle {
        sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform::from_translation(Vec3::new(560.0, -150.0, 890.0)),
        ..default()
    })
    .insert(Enemy)
    .insert(Name::new("Enemy"))
    .insert(Health {
        health: 30,
    })
    .insert(Pathinder {
        vision: 250.0,
        speed: ENEMY_SPEED,
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