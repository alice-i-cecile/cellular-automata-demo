//! Camera controls for the simulation.
//!
//! These can easily be adapted to any 2D simulation or RTS-style game.

use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};
// use bevy_simple_subsecond_system::hot;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (pan_camera, zoom_camera));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// #[hot]
fn pan_camera(
    mut camera_transform: Single<&mut Transform, With<Camera2d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const MOVE_SPEED: f32 = 200.0;

    let move_left =
        keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA);
    let move_right =
        keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD);
    let move_up = keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW);
    let move_down =
        keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS);

    let vertical_movement = match (move_up, move_down) {
        (true, false) => Vec3::Y,
        (false, true) => -Vec3::Y,
        _ => Vec3::ZERO,
    };

    let horizontal_movement = match (move_left, move_right) {
        (true, false) => -Vec3::X,
        (false, true) => Vec3::X,
        _ => Vec3::ZERO,
    };
    let movement = vertical_movement + horizontal_movement;

    if movement != Vec3::ZERO {
        let delta_translation = movement * time.delta_secs() * MOVE_SPEED;
        camera_transform.translation += delta_translation;
    }
}

// #[hot]
fn zoom_camera(
    mut camera_projection: Single<&mut Projection, With<Camera2d>>,
    mousewheel_input: Res<AccumulatedMouseScroll>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    const KEYBOARD_ZOOM_SPEED: f32 = 0.1;
    const MOUSE_ZOOM_SPEED: f32 = 0.05;
    const MIN_ZOOM: f32 = 0.1;
    const MAX_ZOOM: f32 = 10.0;

    let mut zoom = 0.0;
    if keyboard_input.pressed(KeyCode::Equal) || keyboard_input.pressed(KeyCode::NumpadAdd) {
        zoom += KEYBOARD_ZOOM_SPEED;
    }

    if keyboard_input.pressed(KeyCode::Minus) || keyboard_input.pressed(KeyCode::NumpadSubtract) {
        zoom -= KEYBOARD_ZOOM_SPEED;
    }

    zoom += mousewheel_input.delta.y * MOUSE_ZOOM_SPEED;

    if zoom != 0.0 {
        // Thanks Rust: autoderef doesn't work nicely with match statements
        match &mut **camera_projection {
            Projection::Orthographic(ortho) => {
                // We need to invert the sign here to get the desired behavior
                // of zooming in when the mouse wheel is scrolled up.
                ortho.scale -= zoom;
                ortho.scale = ortho.scale.clamp(MIN_ZOOM, MAX_ZOOM);
            }
            _ => {
                error_once!("Zooming is only supported for orthographic projections.");
            }
        }
    }
}
