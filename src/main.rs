mod bomb;
mod textures;

use bomb::*;
use textures::*;
use bevy::prelude::*;

fn main() {
    App::new()
    .add_systems(PreStartup, setup_window)
    .add_systems(PreStartup, setup_images)
    .add_plugins((DefaultPlugins, BombPlugin))
    .run();
}

// windowの初期設定
fn setup_window(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
