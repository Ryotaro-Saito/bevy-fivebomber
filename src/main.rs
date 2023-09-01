mod presentation;
// mod business;

use bevy::prelude::*;
use presentation::PresentationPlugin;
// use business::BusinessPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PresentationPlugin))
        .run();
}
