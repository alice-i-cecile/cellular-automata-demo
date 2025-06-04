//! Generates (and regenerates) the map at the start of each simulation run.
//!
//! The general structure here is helpful to learn from,
//! but unless you're building a grid-based simulation pretty much all of this can be thrown out.

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use bevy_simple_subsecond_system::hot;
use rand::seq::IndexedRandom;
use strum::IntoEnumIterator;

use crate::SimState;
use crate::simulation::TileKind;
use crate::spatial_index::{Position, Tile};

// PERF: these systems would all be faster as exclusive systems to avoid command overhead
pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapSize>()
            .insert_resource(MapSize {
                width: 50,
                height: 50,
            })
            .add_systems(
                OnEnter(SimState::Generate),
                (
                    clean_up_sim_state,
                    spawn_tiles,
                    determine_if_tiles_are_water,
                    randomize_land_tiles,
                )
                    .chain(),
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
    /// The noise threshold in the range of [0, 1] that determines whether a tile is water.
    ///
    /// For a uniform noise distribution, this value maps perfectly to the average percentage of water tiles in the generated map.
    ///
    /// Increasing this value will result in more water tiles being generated,
    /// while decreasing it will result in fewer water tiles.
    /// The value should be in the range [0, 1], where 0 means no water and 1 means all tiles are water.
    fn water_threshold() -> f32 {
        0.3
    }

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
            // Water tiles are generated using a different mechanism
            Water => 0.0,
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

#[hot]
fn clean_up_sim_state(mut commands: Commands, query: Query<Entity, With<Tile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[hot]
fn spawn_tiles(mut commands: Commands, map_size: Res<MapSize>) {
    // PERF: we could speed this up by using spawn_batch
    for x in 0..map_size.width {
        for y in 0..map_size.height {
            let position = Position { x, y };
            let transform = position.to_transform();
            let sprite = Sprite {
                custom_size: Some(Vec2::splat(Position::PIXELS_PER_TILE)),
                ..Default::default()
            };
            let name = Name::new(format!("Tile ({x}, {y})"));

            commands.spawn((Tile, position, sprite, transform, TileKind::Meadow, name));
        }
    }
}

#[hot]
fn determine_if_tiles_are_water(mut tile_query: Query<(&Position, &mut TileKind)>) {
    use noiz::prelude::*;

    // This is an example of perlin noise!
    // noiz is an incredibly powerful library for generating noise,
    // read its docs for more options!
    let noise = Noise::<PerCell<OrthoGrid, Random<UNorm, f32>>>::default();

    for (&position, mut tile_kind) in tile_query.iter_mut() {
        let converted_position = Vec2::new(position.x as f32, position.y as f32);

        // TODO: actually pass in the global rng for deterministic map generation
        let noise_value: f32 = noise.sample(converted_position);

        // If the noise value is below a certain threshold, set the tile to water
        if noise_value < TileKind::water_threshold() {
            *tile_kind = TileKind::Water;
        }
    }
}

// Water tiles are generated using a different mechanism, and should not be altered
#[hot]
fn randomize_land_tiles(mut tile_query: Query<&mut TileKind>, mut rng: GlobalEntropy<WyRand>) {
    let state_weights = TileKind::initial_distribution();

    // PERF: generating multiple random choices at once is significantly faster than generating them one by one.
    for mut tile_kind in tile_query.iter_mut() {
        if *tile_kind != TileKind::Water {
            *tile_kind = state_weights
                .choose_weighted(&mut rng, |item| item.1)
                .unwrap()
                .0;
        }
    }
}

fn finish_generation(mut next_state: ResMut<NextState<SimState>>) {
    info!("Map generation complete, transitioning to Run state");
    next_state.set(SimState::Run);
}

#[hot]
fn regenerate_when_settings_change(
    map_size: Res<MapSize>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    if map_size.is_changed() {
        info!("Map size changed to {:?}, regenerating map", *map_size);
        next_state.set(SimState::Generate);
    }
}
