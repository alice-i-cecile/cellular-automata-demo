use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, pan_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[hot]
fn pan_camera(
    mut camera_transform: Single<&mut Transform, With<Camera2d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const MOVE_SPEED: f32 = 50.0;

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
