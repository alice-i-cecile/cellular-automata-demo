//! Renders the graphics for the simulation.

use bevy::prelude::*;
use bevy_tilemap::TileTextureIndex;

use crate::control_flow::run_simulation;
use crate::simulation::TileKind;

pub struct GraphicsPlugin;

pub const TILE_SIZE: u32 = 32;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_tile_graphics.after(run_simulation));
    }
}

fn update_tile_graphics(
    mut tile_query: Query<(&mut TileTextureIndex, &TileKind), Changed<TileKind>>,
) {
    for (mut tile_texture_index, succession_state) in tile_query.iter_mut() {
        tile_texture_index.0 = succession_state.texture_index();
    }
}

impl TileKind {
    /// The texture index associated with this state.
    ///
    /// This is used to determine the texture of the tile in the map.
    pub fn texture_index(&self) -> u16 {
        use TileKind::*;

        match self {
            Meadow => 0,
            Shrubland => 1,
            ShadeIntolerantForest => 2,
            ShadeTolerantForest => 3,
            Water => 4,
            Fire => 5,
        }
    }
}
