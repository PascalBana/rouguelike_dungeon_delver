use bevy::prelude::*;

mod player;
mod ascii;
mod map;
mod enemy;
mod health;
mod pathfinding;

use player::*;
use ascii::*;
use map::*;
use enemy::*;
use health::*;
use pathfinding::*;

fn main() {
    App::new()                                                         
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(PreStartup, load_ascii)
        .add_systems(
            Startup, (
            spawn_camera, 
            spawn_player, 
            spawn_map, 
            spawn_enemy
        ))
        .add_systems(
        FixedUpdate, (
            player_movement, 
            camera_follow, 
            attack_enemy, 
            health_check,
            move_towards_player,
        ))
        .run(); 
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..default()
    });
}








