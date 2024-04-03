use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, app::AppExit};

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

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, exit_app_on_esc);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { // a bundle is a group of components
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)), // set the background color
        },
        ..default() // use default values for the rest of the components
    });
}

fn exit_app_on_esc(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
} 







