use bevy::prelude::*;

use crate::plugin::MyPlugin;
use crate::resource::{Game, GameState};
use crate::setup::setup;
use crate::systems::{
    animate_bird, blink_space_bar_text, gravity, jump, move_background, move_ground, pipes,
    render_score, score, start_game as start_game_system,
};

#[cfg(target_arch = "wasm32")]
use gloo_timers::future::TimeoutFuture;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

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

/// Build the Bevy app (shared between native + wasm)
pub fn build_app() -> App {
    let mut app = App::new();

    app.init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update, blink_space_bar_text.run_if(is_game_not_active))
        .add_systems(Update, move_background.run_if(is_game_active))
        .add_systems(Update, move_ground.run_if(is_game_active))
        .add_systems(Update, animate_bird.run_if(is_game_active))
        .add_systems(Update, start_game_system.run_if(is_game_not_active))
        .add_systems(Update, gravity.run_if(is_game_active))
        .add_systems(Update, jump.run_if(is_game_active))
        .add_systems(Update, pipes.run_if(is_game_active))
        .add_systems(Update, score.run_if(is_game_active))
        .add_systems(Update, render_score.run_if(is_game_active))
        .add_plugins(MyPlugin);

    app
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start_game() {
    let mut app = build_app();

    app.finish();
    app.cleanup();

    spawn_local(async move {
        loop {
            app.update();
            TimeoutFuture::new(16).await; // ~60 FPS
        }
    });
}
