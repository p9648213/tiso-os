use bevy::prelude::*;

use crate::components::{Background, Bird, Ground, PressSpaceBarText};

pub fn blink_space_bar_text(
    time: Res<Time>,
    mut query: Query<(&mut PressSpaceBarText, &mut Visibility)>,
) {
    let (mut space, mut visibility) = query.single_mut().unwrap();

    let timer = &mut space.0;
    timer.tick(time.delta());

    if timer.finished() {
        if *visibility == Visibility::Hidden {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn move_background(time: Res<Time>, mut query: Query<&mut Transform, With<Background>>) {
    let mut background_transform = query.single_mut().unwrap();
    let delta = time.delta().as_secs_f32();
    let delta_x = 20. * delta;

    background_transform.translation.x -= delta_x;

    if background_transform.translation.x < -288. {
        background_transform.translation.x = 0.;
    }
}

pub fn move_ground(time: Res<Time>, mut query: Query<&mut Transform, With<Ground>>) {
    let mut ground_transform = query.single_mut().unwrap();
    let delta = time.delta().as_secs_f32();
    let delta_x = 150. * delta; // move faster because it's closer to the camera perspective

    ground_transform.translation.x -= delta_x;

    if ground_transform.translation.x < -288. {
        ground_transform.translation.x = 0.;
    }
}

pub fn animate_bird(time: Res<Time>, mut query: Query<(&mut Bird, &mut Sprite)>) {
    for (mut bird, mut sprite) in query.iter_mut() {
        let texture_atlas = sprite.texture_atlas.as_mut().unwrap();
        let delta = time.delta();
        bird.timer.tick(delta);

        if bird.timer.finished() {
            texture_atlas.index = if texture_atlas.index == 2 {
                0
            } else {
                texture_atlas.index + 1
            }
        }
    }
}
