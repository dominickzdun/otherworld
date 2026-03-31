use crate::world::*;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
pub struct ChunkPlugin;
use bevy::mesh::Indices;
use bevy::prelude::Mesh;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_chunk_mesh)
            .add_systems(Update, update_chunk_mesh);
    }
}

fn get_tile_uv(tile_id: u16) -> (f32, f32, f32, f32) {
    // for one row
    let tiles_in_one_row = 8.0;
    let tile_width = 1.0 / tiles_in_one_row;
    let eps = 0.01;

    let u_min = tile_id as f32 * tile_width + eps;
    let u_max = u_min + tile_width - eps * 1.35;

    let v_min = 1.0 + eps * 1.35;
    let v_max = 0.0 - eps * 1.35;
    (u_min, v_min, u_max, v_max)
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

    let px = x as f32 * TILE_SIZE;
    let py = y as f32 * TILE_SIZE;

    positions.push([px, py, 0.0]);
    positions.push([px + TILE_SIZE, py, 0.0]);
    positions.push([px + TILE_SIZE, py + TILE_SIZE, 0.0]);
    positions.push([px, py + TILE_SIZE, 0.0]);

    let (u_min, v_min, u_max, v_max) = get_tile_uv(tile);

    uvs.push([u_min, v_min]);
    uvs.push([u_max, v_min]);
    uvs.push([u_max, v_max]);
    uvs.push([u_min, v_max]);

    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}

fn create_chunk_mesh(
    mut world_data: ResMut<WorldData>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for c in 0..168 {
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_WIDTH {
                let index = (y * CHUNK_WIDTH + x) as usize;
                let tile = world_data.chunks[c].tiles[index];
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
        let mut mesh = Mesh::new(
            bevy::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(indices));

        let mesh_handle = meshes.add(mesh);

        let material_handle = materials.add(ColorMaterial {
            texture: Some(asset_server.load("spritesheet.png")),
            ..default()
        });
        let mesh_entity = commands
            .spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(material_handle),
                Transform::from_xyz(
                    world_data.chunks[c].start_x as f32,
                    world_data.chunks[c].start_y as f32,
                    0.0,
                ),
            ))
            .id();
        world_data.chunks[c].entity = Some(mesh_entity);
    }
}

fn update_chunk_mesh(
    mut world_data: ResMut<WorldData>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if world_data.chunks_for_render.is_empty() {
        return;
    }

    for i in 0..world_data.chunks_for_render.len() {
        let chunk_index = world_data.chunks_for_render[i];
        let chunk = &mut world_data.chunks[chunk_index];

        if let Some(entity) = chunk.entity {
            commands.entity(entity).despawn();
        }

        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_WIDTH {
                let tile_index = (y * CHUNK_WIDTH + x) as usize;
                let tile = chunk.tiles[tile_index];
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
        let mut mesh = Mesh::new(
            bevy::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(indices));

        let mesh_handle = meshes.add(mesh);

        let material_handle = materials.add(ColorMaterial {
            texture: Some(asset_server.load("spritesheet.png")),
            ..default()
        });

        let mesh_entity = commands
            .spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(material_handle),
                Transform::from_xyz(chunk.start_x as f32, chunk.start_y as f32, 0.0),
            ))
            .id();

        chunk.entity = Some(mesh_entity);
    }
    world_data.chunks_for_render = Vec::new();
}
