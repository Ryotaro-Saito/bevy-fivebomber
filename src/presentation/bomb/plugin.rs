use bevy::prelude::*;
use crate::presentation::bomb::component::Bomb;
use crate::presentation::textures::Textures;
use crate::presentation::window::Window;
use crate::view_model_event::bomb_view_model_event::BombViewModelEvent;

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
fn update_bomb(mut query: Query<(&Bomb, &mut Transform)>, window: Res<Window>, mut event_reader: EventReader<BombViewModelEvent>) {
    for event in event_reader.iter() {
        for (bomb, mut transform) in query.iter_mut() {
            let position = bomb.calc_position(event, &window);

            // 逐次処理
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
    }
}