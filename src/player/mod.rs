use bevy::prelude::*;
pub struct PlayerPlugin;
use crate::world::*;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (move_player, update_camera).chain());
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
}

pub fn spawn_player(mut commands: Commands) {
    let player = Player {
        spawn_x: 4150.0 * TILE_SIZE as f32, // Change so players spawns in middle of map
        spawn_y: 1050.0 * TILE_SIZE as f32,
        velocity: Vec2 { x: 0.0, y: 0.0 },
        max_speed: 22.0 * TILE_SIZE as f32,
        acceleration: 68.0 * TILE_SIZE as f32,
        friction: 50.0 * TILE_SIZE as f32,
    };

    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb_u8(0, 0, 255),
            custom_size: Some(Vec2::new(TILE_SIZE as f32 * 2., TILE_SIZE as f32 * 3.)),
            ..default()
        },
        Transform::from_xyz(player.spawn_x, player.spawn_y, 0.),
        player,
    ));
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

    let world_width_px = world_data.width as f32 * TILE_SIZE as f32;
    let world_height_px = world_data.height as f32 * TILE_SIZE as f32;

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
    let player_size = Vec2::new(TILE_SIZE as f32 * 2.0, TILE_SIZE as f32 * 3.0);

    let ground_check_size = Vec2::new(player_size.x - 2.0, player_size.y);
    let is_grounded = is_colliding(
        player_sprite.translation - Vec3::new(0.0, 1.0, 0.0),
        ground_check_size,
        &world_data,
    );
    let delta_time = time.delta_secs();

    let gravity_accel = -90.0 * TILE_SIZE as f32;
    let max_fall_speed = -37.5 * TILE_SIZE as f32;

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
        player_data.velocity.y = 37.0 * TILE_SIZE as f32;
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
        player_sprite.translation.x = next_x.x;
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

fn is_colliding(pos: Vec3, size: Vec2, world: &WorldData) -> bool {
    let half_w = size.x / 2.0;
    let half_h = size.y / 2.0;

    // define the bounding box of the player
    let left = pos.x - half_w;
    let right = pos.x + half_w;
    let bottom = pos.y - half_h;
    let top = pos.y + half_h;

    let grid_x_start = (left / TILE_SIZE as f32).floor() as i32;
    let grid_x_end = (right / TILE_SIZE as f32).floor() as i32;
    let grid_y_start = (bottom / TILE_SIZE as f32).floor() as i32;
    let grid_y_end = (top / TILE_SIZE as f32).floor() as i32;

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
