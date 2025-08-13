use bevy::prelude::*;

use crate::plugin::MyPlugin;
use crate::setup::setup;

mod constants;
mod plugin;
mod setup;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(MyPlugin)
        .run();
}
