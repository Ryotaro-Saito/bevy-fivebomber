pub mod game;
mod question;
mod answer;
mod answer_log;
mod question_repository;

use bevy::prelude::*;

use self::game::plugin::GamePlugin;

pub struct BusinessPlugin;
impl Plugin for BusinessPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(GamePlugin);
  }
}
