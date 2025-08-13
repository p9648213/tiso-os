use bevy::prelude::*;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera
    commands.spawn(Camera2d);

    // Spawn the background
    commands.spawn(Sprite {
        image: asset_server.load("texture/background.png"),
        image_mode: SpriteImageMode::Tiled {
            tile_x: true,       // Only repeat on the x-axis
            tile_y: false,      // No repeat on the y-axis
            stretch_value: 1.0, // No stretching
        },
        custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
        ..default()
    });
}
