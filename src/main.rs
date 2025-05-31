use std::hash::Hash;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use bevy_tilemap::prelude::*;

mod camera;
mod control_flow;
// mod dev_tools;
mod graphics;
mod map_generation;
mod simulation;
mod spatial_index;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // Third-party plugins
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(TilemapPlugin)
        // Crate plugins
        .add_plugins((
            camera::CameraPlugin,
            control_flow::ControlFlowPlugin,
            // dev_tools::DevToolsPlugin,
            graphics::GraphicsPlugin,
            map_generation::MapGenerationPlugin,
            spatial_index::TilePlugin,
            simulation::TransitionPlugin,
        ))
        .init_state::<SimState>()
        .run();
}

#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum SimState {
    #[default]
    Generate,
    Run,
    Paused,
}
