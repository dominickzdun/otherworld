use bevy::{math::f32, prelude::*};
use noise::{NoiseFn, Perlin, Seedable};
const PLAYER_SPEED: f32 = 5000.0;
const TILE_SIZE: f32 = 15.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldData {
            tiles: Vec::new(),
            tiles_loaded: Vec::new(),
            width: 4200,  //4200
            height: 1200, //1200
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_player, load_tiles_around_player, update_camera).chain(),
        )
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
    tiles_loaded: Vec<usize>,
    width: i32,
    height: i32,
}

#[derive(Clone, Debug, PartialEq)]
enum Block {
    None,
    Grass,
    Dirt,
    Stone,
}

#[derive(Clone, Debug)]
struct Tile {
    block: Block,
    loaded: bool,
    entity: Option<Entity>,
}

#[derive(Resource)]
struct TileTextures {
    dirt: Handle<Image>,
    grass: Handle<Image>,
    stone: Handle<Image>,
}

fn load_tiles_around_player(
    player: Single<&Transform, With<Player>>,
    mut commands: Commands,
    mut world_data: ResMut<WorldData>,
    tile_textures: Res<TileTextures>,
) {
    // make an array of indexs that have been loaded
    // use that array of indexs to check if those indexs of world data are still inside player range
    // also load new tile only if player position changes

    const TILE_RADIUS: i32 = 50;

    let player_tile_x = (player.translation.x / TILE_SIZE).floor() as i32;
    let player_tile_y = (player.translation.y / TILE_SIZE).floor() as i32;

    let start_x = player_tile_x - TILE_RADIUS;
    let end_x = player_tile_x + TILE_RADIUS;

    let start_y = player_tile_y - TILE_RADIUS;
    let end_y = player_tile_y + TILE_RADIUS;

    for x in 0..=world_data.width {
        // despawn tiles
        for y in 0..=world_data.height {
            let index = (x * (world_data.height + 1) + y) as usize;

            if world_data.tiles[index].loaded {
                if (x - player_tile_x).abs() > TILE_RADIUS
                    || (y - player_tile_y).abs() > TILE_RADIUS
                {
                    if let Some(entity) = world_data.tiles[index].entity {
                        commands.entity(entity).despawn();
                    }

                    world_data.tiles[index].loaded = false;
                    world_data.tiles[index].entity = None;
                }
            }
        }
    }
    // new despawn method
    // let mut i = 0;

    // while i < world_data.tiles_loaded.len() {
    //     let index = world_data.tiles_loaded[i];

    //     let x = index as i32 / (world_data.height + 1);
    //     let y = index as i32 % (world_data.height + 1);

    //     if (x - player_tile_x).abs() > TILE_RADIUS || (y - player_tile_y).abs() > TILE_RADIUS {
    //         if let Some(entity) = world_data.tiles[index].entity {
    //             commands.entity(entity).despawn();
    //         }

    //         world_data.tiles[index].loaded = false;
    //         world_data.tiles[index].entity = None;

    //         // remove from loaded list
    //         world_data.tiles_loaded.swap_remove(i);
    //     } else {
    //         i += 1;
    //     }
    // }

    for x in start_x..=end_x {
        //draw new loaded tiles
        for y in start_y..=end_y {
            if x < 0 || y < 0 || x > world_data.width || y > world_data.height {
                continue;
            }

            let index = (x * (world_data.height + 1) + y) as usize;

            if world_data.tiles[index].loaded {
                continue;
            }

            if world_data.tiles[index].block == Block::Dirt {
                let world_x = x as f32 * TILE_SIZE;
                let world_y = y as f32 * TILE_SIZE;

                let entity = commands
                    .spawn((
                        Sprite {
                            image: tile_textures.dirt.clone(),
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            ..default()
                        },
                        Transform::from_xyz(world_x, world_y, 0.),
                    ))
                    .id();

                world_data.tiles[index].loaded = true;
                world_data.tiles[index].entity = Some(entity);
                world_data.tiles_loaded.push(index);
            }
        }
    }
}

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
    // println!("x: {} y: {} ", player.translation.x/TILE_SIZE, player.translation.y/TILE_SIZE);
}

fn setup(
    mut commands: Commands,
    mut world_data: ResMut<WorldData>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(TileTextures {
        dirt: asset_server.load("dirtblock.png"),
        grass: asset_server.load("grassblock.png"),
        stone: asset_server.load("stoneblock.png"),
    });

    // Create user
    let mut player = Player {
        x: 42.0 * TILE_SIZE, // Change so players spawns in middle of map
        y: 600.0,
    };

    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb_u8(0, 0, 255),
            custom_size: Some(Vec2::new(TILE_SIZE * 2., TILE_SIZE * 3.)),
            ..default()
        },
        Transform::from_xyz(player.x, player.y, 0.),
        player,
    ));

    let perlin = Perlin::new(5436457);
    // generate world, move tile data into world data, read from world data to load in blocks around player
    // fill world data tiles with blanks
    let size = ((world_data.width + 1) * (world_data.height + 1)) as usize;

    world_data.tiles = vec![
        Tile {
            block: Block::None,
            loaded: false,
            entity: None,
        };
        size
    ];
    // edit world data tiles with dirt blocks and generate world
    for i in 0..=world_data.width {
        let noiseValue = (60.0 * perlin.get([i as f64 * 0.002, 0.0])) // big hills
            + 25.0 * perlin.get([i as f64 * 0.01, 0.0])  // medium hills
            + 5.0 * perlin.get([i as f64 * 0.05, 0.0]); // small hils

        let height_tiles = (200.0 + noiseValue / 90.0 * 50.0) as usize;

        for j in 0..height_tiles {
            let index = i * (world_data.height + 1) + (j as i32);
            if index < (world_data.tiles.len() as i32) {
                world_data.tiles[index as usize].block = Block::Dirt;
            }
        }
    }
}
