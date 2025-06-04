//! Camera controls for the simulation.
//!
//! These can easily be adapted to any 2D simulation or RTS-style game.

use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};
use bevy_egui::input::egui_wants_any_keyboard_input;
use bevy_simple_subsecond_system::hot;

use crate::SimState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (pan_camera, zoom_camera).run_if(not(egui_wants_any_keyboard_input)),
            )
            .add_systems(OnExit(SimState::Generate), adjust_camera_to_map_extents);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[hot]
fn pan_camera(
    mut camera: Single<(&mut Transform, &Projection), With<Camera2d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const PAN_SPEED: f32 = 400.0;

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
        let (camera_transform, camera_projection) = &mut *camera;

        let zoom_level = match &*camera_projection {
            Projection::Orthographic(ortho) => ortho.scale,
            _ => {
                error_once!("Panning is only supported for orthographic projections.");
                return;
            }
        };

        // Scale the camera movement by the delta time to make it frame-rate independent
        // Scale the camera movement by the zoom level to allow easier panning when zoomed out
        let delta_translation = movement * time.delta_secs() * PAN_SPEED * zoom_level;
        camera_transform.translation += delta_translation;
    }
}

#[hot]
fn zoom_camera(
    mut camera_projection: Single<&mut Projection, With<Camera2d>>,
    mousewheel_input: Res<AccumulatedMouseScroll>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    const KEYBOARD_ZOOM_SPEED: f32 = 0.2;
    const MOUSE_ZOOM_SPEED: f32 = 0.1;
    const MIN_ZOOM: f32 = 0.1;
    const MAX_ZOOM: f32 = 30.0;

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

// This system could be simpler and faster, and quickly compute the extents of the map
// based on the map and tile sizes. A more general solution is used here to allow for
// easier reuse and robustness to strange setups.
#[hot]
fn adjust_camera_to_map_extents(
    mut camera: Single<(&mut Transform, &mut Projection), With<Camera2d>>,
    tile_query: Query<(&Sprite, &GlobalTransform)>,
    sprite_assets: Res<Assets<Image>>,
) {
    // Tuning lever value selected based on what looks nice!
    const DEFAULT_ZOOM_LEVEL: f32 = 1.5e-3;

    // Compute the axis-aligned bounding box of the map by examining all tiles
    let mut lower_left = Vec3::new(f32::MAX, f32::MAX, 0.0);
    let mut upper_right = Vec3::new(f32::MIN, f32::MIN, 0.0);

    for (sprite, global_transform) in tile_query.iter() {
        let size = if let Some(size) = sprite.custom_size {
            size
        } else if let Some(image) = sprite_assets.get(&sprite.image) {
            Vec2::new(image.width() as f32, image.height() as f32)
        } else {
            warn_once!("Tile sprite has no custom size and no image has been loaded for it.");
            continue;
        };

        let half_size = size / 2.0;
        let position = global_transform.translation();

        lower_left.x = lower_left.x.min(position.x - half_size.x);
        lower_left.y = lower_left.y.min(position.y - half_size.y);
        upper_right.x = upper_right.x.max(position.x + half_size.x);
        upper_right.y = upper_right.y.max(position.y + half_size.y);
    }

    let center = (lower_left + upper_right) / 2.0;
    let scale = (upper_right - lower_left).length();

    let (camera_transform, camera_projection) = &mut *camera;

    // Center the camera
    camera_transform.translation = Vec3::new(center.x, center.y, camera_transform.translation.z);

    // Adjust the zoom level
    match &mut **camera_projection {
        Projection::Orthographic(ortho) => {
            let new_zoom = scale * DEFAULT_ZOOM_LEVEL;
            info!(
                "Adjusting camera zoom to {new_zoom} based on map extents of {lower_left}, {upper_right}."
            );
            ortho.scale = new_zoom;
        }
        _ => {
            error_once!("Adjusting camera extents is only supported for orthographic projections.");
        }
    }
}
