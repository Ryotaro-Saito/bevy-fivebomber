use bevy::prelude::*;

const BOMB_IMAGE_PATH: &str = "bomb.png";

#[derive(Resource)]
pub struct Textures {
  pub bomb: Handle<Image>,
}

// 画像のロードとResourceとしての挿入
// ここでResourceに追加することでTexturesから画像を取り出すことができる
pub fn setup_images(mut commands: Commands, asset_server: Res<AssetServer>) {
  let textures = Textures {
    bomb: asset_server.load(BOMB_IMAGE_PATH),
  };
  commands.insert_resource(textures);
}
