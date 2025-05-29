use std::hash::Hash;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use strum_macros::EnumIter;

mod camera;
mod dev_tools;
mod graphics;
mod map_generation;
mod transition;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins((
            camera::CameraPlugin,
            dev_tools::DevToolsPlugin,
            graphics::GraphicsPlugin,
            map_generation::MapGenerationPlugin,
            transition::TransitionPlugin,
        ))
        .init_state::<SimState>()
        // Types need to be registered for bevy_inspector_egui
        .register_type::<Position>()
        .register_type::<SuccessionState>()
        .run();
}

#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum SimState {
    #[default]
    Generate,
    Run,
}

#[derive(Component, Reflect)]
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

#[derive(Component, Reflect, PartialEq, Eq, Hash, Debug, Clone, Copy, EnumIter)]
#[require(Sprite)]
enum SuccessionState {
    Meadow,
    Shrubland,
    ShadeIntolerantForest,
    ShadeTolerantForest,
}
