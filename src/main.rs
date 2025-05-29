use std::hash::Hash;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;

mod camera;
mod dev_tools;
mod graphics;
mod map_generation;
mod tile_data;
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
            tile_data::TileDataPlugin,
            transition::TransitionPlugin,
        ))
        .init_state::<SimState>()
        .run();
}

#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum SimState {
    #[default]
    Generate,
    Run,
}
