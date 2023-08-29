use bevy::prelude::*;
use crate::textures::Textures;

// bombに関する処理のPlugin
pub struct BombPlugin;

impl Plugin for BombPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup_bomb);
  }
}

// bombの初期設定
fn setup_bomb(mut commands: Commands, textures: Res<Textures>) {
  // componentの追加
  commands.spawn(Bomb {});

  // spriteの追加
  commands.spawn(SpriteBundle {
    texture: textures.bomb.clone(),
    ..Default::default()
  });
}

// BombのComponent
#[derive(Component)]
struct Bomb;
