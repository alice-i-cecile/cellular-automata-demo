use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        // Types need to be manually registered for bevy-inspector-egui
        app.register_type::<Tile>()
            .register_type::<Position>()
            .init_resource::<TileIndex>()
            .register_type::<TileIndex>();
    }
}

/// A tag component for tiles in the map.
#[derive(Component, Reflect, Default)]
pub struct Tile;

#[derive(Component, Reflect, PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[component(immutable, on_insert = add_position_to_index, on_replace = remove_position_from_index)]
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

    /// Generates the four cardinal neighbors of this position,
    /// to the north, south, east, and west.
    pub fn cardinal_neighbors(&self) -> [Position; 4] {
        [
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
        ]
    }
}

fn add_position_to_index(mut deferred_world: DeferredWorld, hook_context: HookContext) {
    let entity = hook_context.entity;
    let position = deferred_world.get::<Position>(entity).unwrap().clone();

    deferred_world
        .resource_mut::<TileIndex>()
        .tiles
        .insert(position, entity);
}

fn remove_position_from_index(mut deferred_world: DeferredWorld, hook_context: HookContext) {
    let entity = hook_context.entity;
    let position = deferred_world.get::<Position>(entity).unwrap().clone();

    deferred_world
        .resource_mut::<TileIndex>()
        .tiles
        .remove(&position);
}

/// A spatial index that allows you to easily look up tiles by their position.
///
/// It's kept up-to-date via lifecycle hooks on the [`Position`] component,
/// which means that it will automatically update when tiles are added or removed.
/// Because [`Position`] is an immutable component,
/// these values cannot become stale, and the index will always be accurate.
// PERF: note that for most reasonable values of `n` this will still be slower than a linear-time scan,
// because ECS is really really good at those.
// For perf-constrained applications, you should explore other related approaches or work with Bevy itself
// for an optimized first-party solution.
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct TileIndex {
    tiles: HashMap<Position, Entity>,
}

impl TileIndex {
    pub fn get(&self, position: &Position) -> Option<Entity> {
        self.tiles.get(position).copied()
    }
}
