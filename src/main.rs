mod presentation;
mod business;
mod view_model_event;

use bevy::prelude::*;
use presentation::PresentationPlugin;
use business::BusinessPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PresentationPlugin, BusinessPlugin))
        .run();
}
