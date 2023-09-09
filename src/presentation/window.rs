use bevy::prelude::*;

#[derive(Resource)]
pub struct Window {
    pub width: f32,
    pub height: f32,
}

impl Window {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_window);
    }
}

// 画像のロードとResourceとしての挿入
// ここでResourceに追加することでTexturesから画像を取り出すことができる
pub fn setup_window(mut commands: Commands) {
    let window = Window::new(800.0, 600.0);
    commands.insert_resource(window);
}
