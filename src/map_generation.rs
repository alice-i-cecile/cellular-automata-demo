//! Generates (and regenerates) the map at the start of each simulation run.
//!
//! The general structure here is helpful to learn from,
//! but unless you're building a grid-based simulation pretty much all of this can be thrown out.

use bevy::{
    prelude::*,
    sprite::{TileStorage, TilemapLayer, Tileset},
};
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
// use bevy_simple_subsecond_system::hot;
use bevy_tilemap::prelude::*;
use rand::seq::IndexedRandom;
use strum::IntoEnumIterator;

use crate::graphics::TILE_SIZE;
use crate::simulation::TileKind;
use crate::{SimState, spatial_index::Position};

// PERF: these systems would all be faster as exclusive systems to avoid command overhead
pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapSize>()
            .insert_resource(MapSize {
                width: 1280,
                height: 1280,
            })
            .add_systems(
                OnEnter(SimState::Generate),
                (clean_up_sim_state, spawn_tiles).chain(),
            )
            .add_systems(
                Update,
                (
                    regenerate_when_settings_change,
                    finish_generation.run_if(in_state(SimState::Generate)),
                ),
            );
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
struct MapSize {
    width: i32,
    height: i32,
}

impl TileKind {
    /// The non-normalized weight of each state in the initial distribution used to generate the initial map.
    ///
    /// Increasing the weight of a state will increase the likelihood of that state appearing in the initial map.
    /// Decreasing the weight of a state will decrease the likelihood of that state appearing in the initial map.
    /// The weights are not normalized, so they can be any positive value,
    /// or zero to indicate that the state should not appear in the initial map.
    fn initial_distribution_weight(&self) -> f32 {
        use TileKind::*;

        match self {
            Meadow => 1.0,
            Shrubland => 1.0,
            ShadeIntolerantForest => 0.0,
            ShadeTolerantForest => 0.0,
            Water => 0.5,
            Fire => 0.0,
        }
    }

    fn initial_distribution() -> Vec<(TileKind, f32)> {
        let mut vec = Vec::new();

        for variant in TileKind::iter() {
            vec.push((variant, variant.initial_distribution_weight()));
        }

        vec
    }
}

// #[hot]
fn clean_up_sim_state(mut commands: Commands, query: Query<Entity, With<TilemapLayer>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// #[hot]
fn spawn_tiles(
    mut commands: Commands,
    assets: Res<AssetServer>,
    map_size: Res<MapSize>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let state_weights = TileKind::initial_distribution();

    commands
        .spawn((
            TilemapLayer::default(),
            TileStorage::dense(uvec2(map_size.width as u32, map_size.height as u32)),
            TilemapTiles::default(),
            Tileset {
                image: assets.load("basic.tileset.ron"),
                tile_size: UVec2::splat(TILE_SIZE),
            },
            Transform::from_xyz(
                -map_size.width as f32 * TILE_SIZE as f32 / 2.0,
                -map_size.height as f32 * TILE_SIZE as f32 / 2.0,
                0.0,
            ),
        ))
        .with_related_entities::<TileOf>(|t| {
            // PERF: we could speed this up by using spawn_batch
            // PERF: generating multiple random choices at once is significantly faster than generating them one by one.
            for x in 0..map_size.width {
                for y in 0..map_size.height {
                    let position = Position { x, y };
                    let tile_position = TilePosition(ivec2(x, y));
                    let succession_state = state_weights
                        .choose_weighted(&mut rng, |item| item.1)
                        .unwrap()
                        .0;
                    let texture_index = TileTextureIndex(succession_state.texture_index());

                    t.spawn((position, tile_position, succession_state, texture_index));
                }
            }
        });
}

fn finish_generation(mut next_state: ResMut<NextState<SimState>>) {
    info!("Map generation complete, transitioning to Run state");
    next_state.set(SimState::Run);
}

// #[hot]
fn regenerate_when_settings_change(
    map_size: Res<MapSize>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    if map_size.is_changed() {
        info!("Map size changed to {:?}, regenerating map", *map_size);
        next_state.set(SimState::Generate);
    }
}
