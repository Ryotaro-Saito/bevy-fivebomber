use bevy::prelude::*;
use crate::bomb::component::Bomb;
use crate::textures::Textures;

// bombに関する処理のPlugin
pub struct BombPlugin;

impl Plugin for BombPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_bomb)
      .add_systems(Update, update_bomb);
  }
}

// bombの初期設定
fn setup_bomb(mut commands: Commands, textures: Res<Textures>) {
  // sprite/componentの追加
  commands.spawn(SpriteBundle {
    texture: textures.bomb.clone(),
    transform: Transform {
      scale: Vec3::new(0.2, 0.2, 1.),
      ..Default::default()
    },
    ..Default::default()
  })
  .insert(Bomb {});
}

// bombの逐次処理
fn update_bomb(mut query: Query<&mut Transform, With<Bomb>>) {
  for mut transform in query.iter_mut() {
    // 逐次処理
    transform.translation.y -= 1.0;
  }
}