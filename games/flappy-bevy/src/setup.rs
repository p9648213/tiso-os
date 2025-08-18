use bevy::prelude::*;

use crate::{
    components::{
        Background, Bird, GameOverText, Ground, LowerPipe, PressSpaceBarText, ScoreText, UpperPipe,
    },
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    utils::random_pipe_position,
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let number_layout = TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);

    let number_texture_atlas_layout = texture_atlas_layouts.add(number_layout);

    // Spawn a 2D camera
    commands.spawn(Camera2d);

    // Spawn the background
    commands.spawn((
        Sprite {
            image: asset_server.load("texture/background.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,       // Only repeat on the x-axis
                tile_y: false,      // No repeat on the y-axis
                stretch_value: 1.0, // No stretching
            },
            custom_size: Some(Vec2::new(WINDOW_WIDTH + 288. * 2., WINDOW_HEIGHT)),
            ..default()
        },
        Background,
    ));

    // Spawn the ground
    commands.spawn((
        Sprite {
            image: asset_server.load("texture/base.png"),
            custom_size: Some(Vec2::new(WINDOW_WIDTH + 288. * 2., 112.)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,       // Only repeat on the x-axis
                tile_y: false,      // No repeat on the y-axis
                stretch_value: 1.0, // No stretching
            },
            ..default()
        },
        Transform::from_xyz(0., -250., 1.),
        Ground,
    ));

    // Game over text
    commands.spawn((
        Sprite {
            image: asset_server.load("texture/game-over.png"),
            ..default()
        },
        Visibility::Hidden,
        Transform::from_xyz(0., 0., 1.),
        GameOverText,
    ));

    // Space bar text
    commands.spawn((
        Sprite {
            image: asset_server.load("texture/space.png"),
            ..default()
        },
        Transform::from_xyz(0., -50., 1.),
        PressSpaceBarText(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));

    // Score text
    for i in 0..3 {
        let starting_point = -350. + (i as f32 * (24. + 2.)); // 24 is the width + 0.2 is the space between the numbers

        commands.spawn((
            Sprite {
                image: asset_server.load("texture/numbers.png"),
                texture_atlas: Some(TextureAtlas {
                    index: 0,
                    layout: number_texture_atlas_layout.clone(),
                }),
                ..default()
            },
            Transform::from_xyz(starting_point, 200., 1.),
            ScoreText,
        ));
    }

    // Spawn the bird
    commands.spawn((
        Sprite {
            image: asset_server.load("texture/bird.png"),
            texture_atlas: Some(TextureAtlas {
                index: 1,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(34, 24),
                    3,
                    1,
                    None,
                    None,
                )),
            }),
            ..default()
        },
        Transform::from_xyz(0., 0., 2.),
        Bird {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        },
    ));

    for i in 0..5 {
        let delta_x = i as f32 * 200.;
        let (lower_y, upper_y) = random_pipe_position();
        let mut transform = Transform::from_xyz(350. + delta_x, lower_y, 0.5);

        // Spawn lower pipe
        commands.spawn((
            Sprite {
                image: asset_server.load("texture/pipe.png"),
                ..default()
            },
            transform,
            LowerPipe,
        ));

        // Rotate the upper pipe
        transform.rotate(Quat::from_rotation_z(std::f32::consts::PI));
        // Changing the y position of the upper pipe
        transform.translation.y = upper_y;

        // Spawn upper pipe
        commands.spawn((
            Sprite {
                image: asset_server.load("texture/pipe.png"),
                ..default()
            },
            transform,
            UpperPipe,
        ));
    }
}
