
use bevy::prelude::*;

#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

pub fn load_ascii(
    mut commands:Commands,
    assest_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assest_server.load("Ascii.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(9.0, 9.0),
        16,
        16,
        Some(Vec2::new(2.0, 2.0)),
        Some(Vec2::new(0.0, 0.0)),
    );

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(AsciiSheet(atlas_handle))
}
