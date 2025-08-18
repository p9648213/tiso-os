use bevy::prelude::*;

use crate::plugin::MyPlugin;
use crate::setup::setup;
use crate::systems::{animate_bird, blink_space_bar_text, move_background, move_ground};

mod components;
mod constants;
mod plugin;
mod setup;
mod systems;
mod utils;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, blink_space_bar_text)
        .add_systems(Update, move_background)
        .add_systems(Update, move_ground)
        .add_systems(Update, animate_bird)
        .add_plugins(MyPlugin)
        .run();
}
