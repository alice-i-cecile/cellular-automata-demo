use bevy::prelude::*;
use strum_macros::EnumIter;

pub struct TileDataPlugin;

impl Plugin for TileDataPlugin {
    fn build(&self, app: &mut App) {
        // Types need to be manually registered for bevy-inspector-egui
        app.register_type::<Tile>()
            .register_type::<Position>()
            .register_type::<TileKind>();
    }
}

/// A tag component for tiles in the map.
#[derive(Component, Reflect, Default)]
pub struct Tile;

#[derive(Component, Reflect)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const PIXELS_PER_TILE: f32 = 32.0;

    pub fn to_transform(&self) -> Transform {
        Transform::from_xyz(
            self.x as f32 * Self::PIXELS_PER_TILE,
            self.y as f32 * Self::PIXELS_PER_TILE,
            0.0,
        )
    }
}

#[derive(Component, Reflect, PartialEq, Eq, Hash, Debug, Clone, Copy, EnumIter)]
pub enum TileKind {
    Meadow,
    Shrubland,
    ShadeIntolerantForest,
    ShadeTolerantForest,
    Water,
    Fire,
}
