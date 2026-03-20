use crate::player::*;
use crate::world::*;
use bevy::prelude::*;
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (create_chunk_meshes).chain());
    }
}

fn create_chunk_meshes(world_data: Res<WorldData>) {
    for c in 0..world_data.chunks.len() {
        create_chunk_mesh(c, &world_data);
    }
}

fn get_tile_uv(tile_id: u16) -> (f32, f32, f32, f32) {
    // for one row
    let tile_width = 1.0 / 8.0;

    let u_min = tile_id as f32 * tile_width;
    let u_max = u_min + tile_width;

    (u_min, 0.0, u_max, 1.0)
}

// fn get_tile_uv(tile_id: u16) -> (f32, f32, f32, f32) { //MULTIPLE ROWS
//     let tiles_per_row = 8;

//     let tile_size = 1.0 / tiles_per_row as f32;

//     let x = tile_id % tiles_per_row;
//     let y = tile_id / tiles_per_row;

//     let u_min = x as f32 * tile_size;
//     let v_min = y as f32 * tile_size;

//     let u_max = u_min + tile_size;
//     let v_max = v_min + tile_size;

//     (u_min, v_min, u_max, v_max)
// }

fn add_tile_quad(
    x: usize,
    y: usize,
    tile: u16,
    positions: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
) {
    let base = positions.len() as u32;

    let px = x as f32 * TILE_SIZE as f32;
    let py = y as f32 * TILE_SIZE as f32;

    positions.push([px, py, 0.0]);
    positions.push([px + TILE_SIZE as f32, py, 0.0]);
    positions.push([px + TILE_SIZE as f32, py + TILE_SIZE as f32, 0.0]);
    positions.push([px, py + TILE_SIZE as f32, 0.0]);

    let (u_min, v_min, u_max, v_max) = get_tile_uv(tile);

    uvs.push([u_min, v_min]);
    uvs.push([u_max, v_min]);
    uvs.push([u_max, v_max]);
    uvs.push([u_min, v_max]);

    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}

fn create_chunk_mesh(chunk_index: usize, world_data: &Res<WorldData>) {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for y in 0..CHUNK_HEIGHT {
        for x in 0..CHUNK_WIDTH {
            let index = (y * CHUNK_WIDTH + x) as usize;
            let tile = world_data.chunks[chunk_index].tiles[index];
            if tile == AIR {
                continue;
            }
            add_tile_quad(
                x as usize,
                y as usize,
                tile,
                &mut positions,
                &mut uvs,
                &mut indices,
            );
        }
    }
}
