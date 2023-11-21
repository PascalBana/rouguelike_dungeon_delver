use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

mod player;
mod ascii;
mod map;
mod enemy;
mod health;
mod pathfinding;
mod gamestate;
mod splash;
mod menu;

use player::*;
use ascii::*;
use map::*;
use enemy::*;
use health::*;
use pathfinding::*;
use gamestate::*;
use splash::*;
use menu::*;

fn main() {
    App::new()                                                         
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            StartupPlugin,
            SplashPlugin,
            MenuPlugin,
            AsciiPlugin,
            MapPlugin,
            PlayerPlugin,
            EnemyPlugin,
            HealthPlugin,
            PathfinderPlugin,
        ))
        .add_state::<GameState>()
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
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
        },
        ..default()
    });
}








