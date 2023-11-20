use bevy::prelude::*;

mod player;
mod ascii;
mod map;
mod enemy;
mod health;
mod pathfinding;
mod gamestate;

use player::*;
use ascii::*;
use map::*;
use enemy::*;
use health::*;
use pathfinding::*;
use gamestate::*;

fn main() {
    App::new()                                                         
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            StartupPlugin,
            AsciiPlugin,
            MapPlugin,
            PlayerPlugin,
            EnemyPlugin,
            HealthPlugin,
            PathfinderPlugin,
        ))

        .run(); 
}

// create title screen with a play button in the center to start the game

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..default()
    });
}








