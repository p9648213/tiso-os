use bevy::prelude::*;

use crate::plugin::MyPlugin;
use crate::resource::{Game, GameState};
use crate::setup::setup;
use crate::systems::{
    animate_bird, blink_space_bar_text, gravity, jump, move_background, move_ground, pipes,
    render_score, score, start_game,
};

mod components;
mod constants;
mod plugin;
mod resource;
mod setup;
mod systems;
mod utils;

fn is_game_active(game: Res<Game>) -> bool {
    game.state == GameState::Active
}

fn is_game_not_active(game: Res<Game>) -> bool {
    game.state != GameState::Active
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update, blink_space_bar_text.run_if(is_game_not_active))
        .add_systems(Update, move_background.run_if(is_game_active))
        .add_systems(Update, move_ground.run_if(is_game_active))
        .add_systems(Update, animate_bird.run_if(is_game_active))
        .add_systems(Update, start_game.run_if(is_game_not_active))
        .add_systems(Update, gravity.run_if(is_game_active))
        .add_systems(Update, jump.run_if(is_game_active))
        .add_systems(Update, pipes.run_if(is_game_active))
        .add_systems(Update, score.run_if(is_game_active))
        .add_systems(Update, render_score.run_if(is_game_active))
        .add_plugins(MyPlugin)
        .run();
}
