fn load_tiles_around_player(
    player: Single<&Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_data: ResMut<WorldData>,
    tile_textures: Res<TileTextures>,
) {
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
            }
        }
    }
}

// use hash map to store values of indexs of loaded tiles
// if index is in hash map -> skip
// if index is not in hash map -> load tile and add value to hash map
// at the end if the indexs created by the player surroundings aren't the same as the indexs in the hashmap -> delete indexs and destroy tiles
