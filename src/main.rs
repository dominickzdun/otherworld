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
    //settings.limiter = Limiter::from_framerate(60.0);
}
