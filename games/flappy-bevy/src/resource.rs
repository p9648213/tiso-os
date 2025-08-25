use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Game {
    pub score: u32,
    pub state: GameState,
}

#[derive(PartialEq, Default)]
pub enum GameState {
    Active,
    #[default]
    Inactive,
    GameOver,
}
