pub mod bomb;
mod textures;

use bevy::prelude::*;

use self::textures::setup_images;

pub struct PresentationPlugin;
impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bomb::plugin::BombPlugin)
            .add_systems(PreStartup, setup_window)
            .add_systems(PreStartup, setup_images);
    }
}

// windowの初期設定
fn setup_window(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
