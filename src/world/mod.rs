use bevy::prelude::*;

pub const CHUNK_WIDTH: i32 = 200;
pub const CHUNK_HEIGHT: i32 = 150;
pub const TILE_SIZE: i32 = 16;

pub const AIR: u16 = 3;
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
        solid: true,
        atlas_index: STONE,
    }, // stone
    TileDef {
        solid: true,
        atlas_index: DIRT,
    }, // dirt
    TileDef {
        solid: true,
        atlas_index: GRASS,
    }, // grass
    TileDef {
        solid: false,
        atlas_index: AIR,
    },
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
    pub fn get_tile_id(&self, grid_x: f32, grid_y: f32) -> u16 {
        let chunks_wide = self.width / CHUNK_WIDTH;

        let chunk_x = (grid_x / CHUNK_WIDTH as f32).floor() as i32;
        let chunk_y = (grid_y / CHUNK_HEIGHT as f32).floor() as i32;

        let chunk_index = (chunk_y * chunks_wide + chunk_x) as usize;

        if chunk_index >= self.chunks.len() {
            return AIR;
        }

        let local_x = (grid_x as i32).rem_euclid(CHUNK_WIDTH) as usize;
        let local_y = (grid_y as i32).rem_euclid(CHUNK_HEIGHT) as usize;

        let tile_index = local_y * CHUNK_WIDTH as usize + local_x;
        self.chunks[chunk_index].tiles[tile_index]
    }

    pub fn is_tile_solid(&self, grid_x: f32, grid_y: f32) -> bool {
        let id = self.get_tile_id(grid_x, grid_y);
        TILE_DEFS[id as usize].solid
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
                let mut set_grass = false;
                let noise_value = (60.0 * perlin.get([world_x as f64 * 0.002, 0.0]))
                    + 25.0 * perlin.get([world_x as f64 * 0.01, 0.0])
                    + 5.0 * perlin.get([world_x as f64 * 0.05, 0.0]);

                let height_tiles = (1000.0 + noise_value / 90.0 * 50.0) as i32;

                for local_y in 0..CHUNK_HEIGHT {
                    let world_y = y + local_y;
                    let index = (local_y * CHUNK_WIDTH + local_x) as usize;

                    if world_y == height_tiles - 1 {
                        tiles[index] = GRASS;
                    } else if world_y < height_tiles - 1 && world_y > height_tiles - 50 {
                        tiles[index] = DIRT;
                    } else if world_y <= height_tiles - 50 {
                        tiles[index] = STONE;
                    } else {
                        tiles[index] = AIR;
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
