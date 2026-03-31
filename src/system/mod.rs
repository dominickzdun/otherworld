#[derive(Resource)]
pub struct SystemSettings {
    pub resolution: Vec2,
    pub fullscreen: bool,
    pub zoom: f32, //from 100% to 200%
}

impl Plugin for SystemSettingsPlugin {}
