use bevy::{prelude::*, sprite::collide_aabb::collide};
use crate::{ascii::AsciiSheet, 
    health::Health, 
    pathfinding::Pathinder, 
    player::{Player, PLAYER_SIZE},
    gamestate::GameState, map::GameLevel,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                FixedUpdate, (
                attack_player,
            ).run_if(in_state(GameState::Game(GameLevel::Level1))));
    }
}

pub const ENEMY_SIZE: f32 = 50.0;
const ENEMY_SPEED: f32 = 40.0;

#[derive(Component)]
pub struct Enemy {
    timer: Timer,
}

pub fn spawn_enemy(
    commands: &mut Commands,
    ascii: &Res<AsciiSheet>,
    spawn_point: Vec3,
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
        transform: Transform::from_translation(spawn_point),
        ..default()
    })
    .insert(Enemy{
        timer: Timer::from_seconds(1.5, TimerMode::Repeating)
    })
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


// if within range, attack player every 2 seconds
pub fn attack_player(
    mut enemy_transform: Query<(&Transform, &mut Enemy)>,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    time: Res<Time>,
) {
    let (player_transform, mut health) = player_query.single_mut();
    for (enemy_transform, mut enemy) in enemy_transform.iter_mut() {
        let collision = collide(
            enemy_transform.translation,
            Vec2::splat(ENEMY_SIZE * 1.2),
            player_transform.translation,
            Vec2::splat(PLAYER_SIZE),
        );
        if collision.is_some() && enemy.timer.tick(time.delta()).just_finished(){
            health.health -= 1;
            println!("Player Health: {}", health.health);
        }
    }
}
