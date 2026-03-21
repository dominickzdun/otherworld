use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_framepace::FramepaceSettings;
use bevy_framepace::Limiter;

mod chunks;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(chunks::ChunkPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut settings: ResMut<FramepaceSettings>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // commands.insert_resource(TileTextures {
    //     dirt: asset_server.load("dirtblock.png"),
    //     grass: asset_server.load("grassblock.png"),
    //     stone: asset_server.load("stoneblock.png"),
    // });

    settings.limiter = Limiter::from_framerate(60.0);
    // let sprite_sheet = asset_server.load::<Image>("spritesheet.png");
    // let layout =
    //     TextureAtlasLayout::from_grid(UVec2::splat(world::TILE_SIZE as u32), 3, 1, None, None);
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // let material_handle = materials.add(ColorMaterial {
    //     texture: Some(sprite_sheet),
    //     ..default()
    // });

    // commands.insert_resource(TileMaterial(material_handle));
}
