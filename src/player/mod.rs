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
    pub previous_chunk: usize,
    pub speed: f32,
}

pub fn spawn_player(mut commands: Commands) {
    let player = Player {
        spawn_x: 4150.0 * TILE_SIZE as f32, // Change so players spawns in middle of map
        spawn_y: 800.0 * TILE_SIZE as f32,
        previous_chunk: 0, //change to starting chunk
        speed: 45.0 * TILE_SIZE as f32,
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
    player_data: Single<&mut Player>,
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

    let move_delta = direction.normalize_or_zero() * player_data.speed * time.delta_secs();
    player_sprite.translation += move_delta.extend(0.);
    println!(
        "{} is x and {} is y",
        player_sprite.translation.x / TILE_SIZE as f32,
        player_sprite.translation.y / TILE_SIZE as f32
    );
}
