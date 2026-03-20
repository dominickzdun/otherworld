use bevy::prelude::*;
pub struct PlayerPlugin;
use crate::world::TILE_SIZE;
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
        spawn_x: 42.0 * 15.0, // Change so players spawns in middle of map
        spawn_y: 1200.0,
        previous_chunk: 0, //change to starting chunk
        speed: 5000.0,
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
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
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
        player_sprite.translation.x, player_sprite.translation.y
    );
}
