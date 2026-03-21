use bevy::prelude::*;

pub const CHUNK_WIDTH: i32 = 200;
pub const CHUNK_HEIGHT: i32 = 150;
pub const TILE_SIZE: i32 = 4;

pub const AIR: u16 = 4;
pub const GRASS: u16 = 2;
pub const DIRT: u16 = 1;
pub const STONE: u16 = 0;

pub type TileId = u16;

#[derive(Clone)]
struct TileDef {
    solid: bool,
    atlas_index: u16,
}

static TILE_DEFS: [TileDef; 4] = [
    TileDef {
        solid: false,
        atlas_index: 4,
    }, // air
    TileDef {
        solid: true,
        atlas_index: 0,
    }, // stone
    TileDef {
        solid: true,
        atlas_index: 1,
    }, // dirt
    TileDef {
        solid: true,
        atlas_index: 2,
    }, // grass
];

pub struct Chunk {
    pub tiles: Vec<TileId>,
    pub dirty: bool,
    pub start_x: f32,
    pub start_y: f32,
}

#[derive(Resource)]
pub struct WorldData {
    pub chunks: Vec<Chunk>,
    pub chunks_loading_marked: Vec<usize>,
    pub chunks_loaded: Vec<usize>,
    pub width: i32,
    pub height: i32,
    seed: u32,
}

impl WorldData {
    pub fn new(seed: u32, width: i32, height: i32) -> Self {
        Self {
            chunks: Vec::new(),
            chunks_loaded: Vec::new(),
            chunks_loading_marked: Vec::new(),
            width: width,
            height: height,
            seed: seed,
        }
    }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldData::new(192852, 4200, 1200))
            .add_systems(Startup, generate_world);
    }
}

pub fn generate_world(mut world_data: ResMut<WorldData>) {
    use noise::{NoiseFn, Perlin};
    let perlin = Perlin::new(world_data.seed);
    // generate world, move tile data into world data, read from world data to load in blocks around player
    // fill world data tiles with blanks
    for y in (0..=world_data.height - CHUNK_HEIGHT).step_by(CHUNK_HEIGHT as usize) {
        for x in (0..=world_data.width - CHUNK_WIDTH).step_by(CHUNK_WIDTH as usize) {
            let mut tiles = vec![AIR; (CHUNK_WIDTH * CHUNK_HEIGHT) as usize];

            for local_x in 0..CHUNK_WIDTH {
                let world_x = x + local_x;

                let noise_value = (60.0 * perlin.get([world_x as f64 * 0.002, 0.0]))
                    + 25.0 * perlin.get([world_x as f64 * 0.01, 0.0])
                    + 5.0 * perlin.get([world_x as f64 * 0.05, 0.0]);

                let height_tiles = (200.0 + noise_value / 90.0 * 50.0) as i32;

                for local_y in 0..CHUNK_HEIGHT {
                    let world_y = y + local_y;

                    if world_y < height_tiles {
                        let index = (local_y * CHUNK_WIDTH + local_x) as usize;

                        tiles[index] = DIRT;
                    }
                }
            }

            let chunk = Chunk {
                tiles: tiles,
                dirty: false,
                start_x: (x * TILE_SIZE) as f32,
                start_y: (y * TILE_SIZE) as f32,
            };

            world_data.chunks.push(chunk);
        }
    }
}
