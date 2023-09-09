use bevy::prelude::*;

use crate::{business::question_repository::component::DummyQuestionRepository, view_model_event::bomb_view_model_event::BombViewModelEvent};

use super::component::Game;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_game)
            .add_event::<BombViewModelEvent>()
            .add_systems(Update, event_emitter);

    }
}

fn setup_game(mut commands: Commands) {
    let question_repository = DummyQuestionRepository::new();
    let game = Game::new(question_repository);
    commands.insert_resource(game);
}

fn event_emitter(mut game: ResMut<Game<DummyQuestionRepository>>, mut event_writer: EventWriter<BombViewModelEvent>, ) {
    BombViewModelEvent::create(
        false,
        0.0,
        0
    ).and_then(|event| { event_writer.send(event); None::<()> });
}
