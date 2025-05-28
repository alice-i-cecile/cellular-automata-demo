use std::hash::Hash;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use strum_macros::EnumIter;

mod camera;
mod graphics;
mod map_generation;
mod transition;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins((
            camera::CameraPlugin,
            graphics::GraphicsPlugin,
            map_generation::MapGenerationPlugin,
            transition::TransitionPlugin,
        ))
        .run();
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const PIXELS_PER_TILE: f32 = 32.0;

    fn to_transform(&self) -> Transform {
        Transform::from_xyz(
            self.x as f32 * Self::PIXELS_PER_TILE,
            self.y as f32 * Self::PIXELS_PER_TILE,
            0.0,
        )
    }
}

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy, EnumIter)]
#[require(Sprite)]
enum SuccessionState {
    Meadow,
    Shrubland,
    ShadeIntolerantForest,
    ShadeTolerantForest,
}
