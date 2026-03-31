use bevy::prelude::*;
pub struct PlayerPlugin;
use crate::world::*;
pub const WEST: u16 = 1;
pub const EAST: u16 = 0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (move_player, update_camera, place_block).chain());
    }
}

#[derive(Component)]
pub struct Player {
    pub spawn_x: f32,
    pub spawn_y: f32,
    pub velocity: Vec2,
    pub max_speed: f32,
    pub acceleration: f32,
    pub friction: f32,
    pub direction: u16,
    pub inventory: Vec<u16>,
    pub selected_item: usize,
}

pub fn spawn_player(mut commands: Commands) {
    let mut player = Player {
        spawn_x: 4150.0 * TILE_SIZE, // Change so players spawns in middle of map
        spawn_y: 1050.0 * TILE_SIZE,
        velocity: Vec2 { x: 0.0, y: 0.0 },
        max_speed: 22.0 * TILE_SIZE,
        acceleration: 68.0 * TILE_SIZE,
        friction: 50.0 * TILE_SIZE,
        direction: EAST,
        inventory: Vec::new(),
        selected_item: 0,
    };

    player.inventory.push(STONE);
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb_u8(0, 0, 255),
            custom_size: Some(Vec2::new(TILE_SIZE * 2., TILE_SIZE * 3.)),
            ..default()
        },
        Transform::from_xyz(player.spawn_x, player.spawn_y, 0.),
        player,
    ));
}

fn place_block(
    window: Single<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    camera: Single<&Transform, With<Camera2d>>,
    player_data: Single<&Player>,
    mut world_data: ResMut<WorldData>,
) {
    // get mouse position,
    // convert to tile local x and y of a chunk
    // check if air
    // place block depending on that
    //
    if let Some(cursor_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let screen_pos = Vec2::new(
            cursor_position.x - window_size.x / 2.0,
            window_size.y / 2.0 - cursor_position.y,
        );
        let world_pos = camera.translation.truncate() + screen_pos;

        let tile_x = (world_pos.x / TILE_SIZE as f32).floor() as i32;
        let tile_y = (world_pos.y / TILE_SIZE as f32).floor() as i32;

        let chunk_x = tile_x.div_euclid(CHUNK_WIDTH as i32);
        let chunk_y = tile_y.div_euclid(CHUNK_HEIGHT as i32);

        let local_x = tile_x.rem_euclid(CHUNK_WIDTH as i32);
        let local_y = tile_y.rem_euclid(CHUNK_HEIGHT as i32);

        // println!("world: {}, {}", local_x, local_y);

        if mouse.just_pressed(MouseButton::Left) {
            let chunks_wide = world_data.width / CHUNK_WIDTH as i32;
            let chunks_high = world_data.height / CHUNK_HEIGHT as i32;

            if chunk_x < 0 || chunk_x >= chunks_wide || chunk_y < 0 || chunk_y >= chunks_high {
                return;
            }

            let chunk_index = (chunk_y * chunks_wide + chunk_x) as usize;

            let chunk = &mut world_data.chunks[chunk_index];

            let tile_index = (local_y * CHUNK_WIDTH as i32 + local_x) as usize;

            chunk.tiles[tile_index] = player_data.inventory[player_data.selected_item];
            world_data.chunks_for_render.push(chunk_index);
            println!("placed block in chunk {}", chunk_index);
        }
    } else {
        println!("not there");
    }
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    world_data: Res<WorldData>,
) {
    //assuming player has 1920x1080 display
    let view_width = 1920.0;
    let view_height = 1080.0;

    let half_width = view_width / 2.0;
    let half_height = view_height / 2.0;

    let world_width_px = world_data.width as f32 * TILE_SIZE;
    let world_height_px = world_data.height as f32 * TILE_SIZE;

    let mut target_x = player.translation.x;
    let mut target_y = player.translation.y;

    if target_x < half_width {
        target_x = half_width;
    } else if target_x > world_width_px - half_width {
        target_x = world_width_px - half_width;
    }

    if target_y < half_height {
        target_y = half_height;
    } else if target_y > world_height_px - half_height {
        target_y = world_height_px - half_height;
    }

    camera.translation.x = target_x;
    camera.translation.y = target_y;
    camera.translation.z = player.translation.z;
}
fn move_player(
    mut player_sprite: Single<&mut Transform, With<Player>>,
    mut player_data: Single<&mut Player>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
    world_data: Res<WorldData>,
) {
    let player_size = Vec2::new((TILE_SIZE * 2.0) - 2., (TILE_SIZE * 3.0) - 2.);

    let ground_check_size = Vec2::new(player_size.x - 2.0, player_size.y);
    let is_grounded = is_colliding(
        player_sprite.translation - Vec3::new(0.0, 1.0, 0.0),
        ground_check_size,
        &world_data,
    );
    let delta_time = time.delta_secs();

    let gravity_accel = -90.0 * TILE_SIZE;
    let max_fall_speed = -37.5 * TILE_SIZE;

    player_data.velocity.y += gravity_accel * delta_time;

    let mut x_input = 0.0;
    if kb_input.pressed(KeyCode::KeyA) {
        x_input -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        x_input += 1.0;
    }

    if x_input != 0.0 {
        player_data.velocity.x += x_input * player_data.acceleration * delta_time;
    } else {
        let friction_amount = player_data.friction * delta_time;
        if player_data.velocity.x.abs() > friction_amount {
            player_data.velocity.x -= player_data.velocity.x.signum() * friction_amount;
        } else {
            player_data.velocity.x = 0.0;
        }
    }

    if kb_input.just_pressed(KeyCode::Space) && is_grounded {
        player_data.velocity.y = 37.0 * TILE_SIZE;
    }

    player_data.velocity.x = player_data
        .velocity
        .x
        .clamp(-player_data.max_speed, player_data.max_speed);
    if player_data.velocity.y < max_fall_speed {
        player_data.velocity.y = max_fall_speed;
    }
    let move_delta = player_data.velocity * delta_time;

    let mut next_x = player_sprite.translation;
    next_x.x += move_delta.x;
    if !is_colliding(next_x, player_size, &world_data) {
        // Normal horizontal movement
        player_sprite.translation.x = next_x.x;
    } else if is_grounded {
        handle_one_block_step(
            &mut player_sprite,
            &mut player_data,
            &world_data,
            move_delta,
            player_size,
        );
    } else {
        player_data.velocity.x = 0.0;
    }

    let mut next_y = player_sprite.translation;
    next_y.y += move_delta.y;
    if !is_colliding(next_y, player_size, &world_data) {
        player_sprite.translation.y = next_y.y;
    } else {
        player_data.velocity.y = 0.0;
    }
}

fn handle_one_block_step(
    transform: &mut Transform,
    player: &mut Player,
    world: &WorldData,
    move_delta: Vec2,
    player_size: Vec2,
) {
    let step_up_height = TILE_SIZE;
    let mut step_up_pos = transform.translation;
    step_up_pos.y += step_up_height;

    // check if player will bump head
    if !is_colliding(step_up_pos, player_size, world) {
        let mut forward_step_pos = step_up_pos;
        forward_step_pos.x += move_delta.x;
        // check if player can move forward
        if !is_colliding(forward_step_pos, player_size, world) {
            transform.translation.y += step_up_height;
            transform.translation.x = forward_step_pos.x;
        } else {
            player.velocity.x = 0.0;
        }
    } else {
        player.velocity.x = 0.0;
    }
}

fn is_colliding(pos: Vec3, size: Vec2, world: &WorldData) -> bool {
    let half_w = size.x / 2.0;
    let half_h = size.y / 2.0;

    // define the bounding box of the player
    let left = pos.x - half_w;
    let right = pos.x + half_w;
    let bottom = pos.y - half_h;
    let top = pos.y + half_h;

    let grid_x_start = (left / TILE_SIZE).floor() as i32;
    let grid_x_end = (right / TILE_SIZE).floor() as i32;
    let grid_y_start = (bottom / TILE_SIZE).floor() as i32;
    let grid_y_end = (top / TILE_SIZE).floor() as i32;

    for y in grid_y_start..=grid_y_end {
        for x in grid_x_start..=grid_x_end {
            if x < 0 || x >= world.width || y < 0 || y >= world.height {
                return true;
            }

            if world.is_tile_solid(x as f32, y as f32) {
                return true;
            }
        }
    }

    false
}
