use bevy::ecs::world;
use bevy::window::PrimaryWindow;
use bevy::{math::f32, prelude::*};
use noise::{NoiseFn, Perlin, Seedable};

const PLAYER_SPEED: f32 = 5000.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldData { tiles: Vec::new() })
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, update_camera).chain())
        .run();
}

#[derive(Component)]
struct Player {
    x: f32,
    y: f32,
}

#[derive(Resource)]
struct WorldData {
    tiles: Vec<Tile>,
}

#[derive(Clone, Debug, PartialEq)]
enum Block {
    None,
    Dirt,
}

#[derive(Clone, Debug)]
struct Tile {
    block: Block,
    x: f32,
    y: f32,
}

fn draw_player() {}

fn draw() {}

fn generate_world(seed: u32) {}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let direction = Vec3::new(player.translation.x, player.translation.y, 0.0);

    camera
        .translation
        .smooth_nudge(&direction, 100., time.delta_secs());
}

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }


    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.);
    // println!("x: {} y: {} ", player.translation.x/15.0, player.translation.y/15.0);
}

fn setup(
    mut commands: Commands,
    mut world_data: ResMut<WorldData>,
    asset_server: Res<AssetServer>,
) {
    let tile_size = 15.0;

    // Create user
    let mut player = Player {
        x: 42.0 * 15.0, // Change so players spawns in middle of map
        y: 600.0,
    };

    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb_u8(0, 0, 255),
            custom_size: Some(Vec2::new(tile_size * 2., tile_size * 3.)),
            ..default()
        },
        Transform::from_xyz(player.x, player.y, 0.),
        player,
    ));

    let perlin = Perlin::new(5436457);
    let width = 500; //4200
    let height = 500; //1200
    let scale = 0.10;
    let max_hill_height = 5.0;
    // generate world, move tile data into world data, read from world data to load in blocks around player
    // fill world data tiles with blanks
    for i in 0..=width {
        for j in 0..=height {
            let tile = Tile {
                block: Block::None,
                x: i as f32 * tile_size,
                y: j as f32 * tile_size,
            };
            world_data.tiles.push(tile);
        }
    }
    // edit world data tiles with dirt blocks and generate world
    for i in 0..=width {
        let value = perlin.get([i as f64 * scale, 0.0]);
        let normalized = (value + 1.0) / 2.0;
        let num_tiles = (normalized * max_hill_height as f64) as usize;

        for j in 0..num_tiles {
            let index = i * (height + 1) + j;
            if index < world_data.tiles.len() {
                world_data.tiles[index].block = Block::Dirt;
            }
        }
    }
    // read from world data and display
    for i in 0..=width  {
        for j in 0..=height {
            let index = i * (height + 1) + j as usize;
            if world_data.tiles[index].block == Block::Dirt {
                commands.spawn((
                    Sprite {
                        image: asset_server.load("dirtblock.png"),
                        image_mode: SpriteImageMode::Auto,
                        custom_size: Some(Vec2::splat(tile_size)),
                        ..default()
                    },
                    Transform::from_xyz(i as f32 * tile_size, j as f32 * tile_size, 0.),
                ));
            }
        }
    }

    // let grid_width = (width / tile_size) as i32;
    // let grid_height = (height / tile_size) as i32;

    // for x in 0..grid_width {
    //     for y in 0..grid_height {
    //         commands.spawn((
    //             Sprite {
    //                 color: Color::srgb_u8(200, 50, 200),
    //                 custom_size: Some(Vec2::splat(tile_size)),
    //                 ..default()
    //             },
    //             Transform::from_xyz(
    //                 x as f32 * tile_size,
    //                 y as f32 * tile_size,
    //                 0.,
    //             ),
    //         ));
    //     }
    // }
}
