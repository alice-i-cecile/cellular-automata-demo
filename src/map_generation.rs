//! Generates (and regenerates) the map at the start of each simulation run.
//!
//! The general structure here is helpful to learn from,
//! but unless you're building a grid-based simulation pretty much all of this can be thrown out.

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use bevy_simple_subsecond_system::hot;
use rand::RngCore;
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
            .init_resource::<MapSize>()
            .register_type::<InitialWeights>()
            .init_resource::<InitialWeights>()
            .register_type::<WaterThreshold>()
            .init_resource::<WaterThreshold>()
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

impl Default for MapSize {
    fn default() -> Self {
        // Default map size is 100x100 tiles
        Self {
            width: 50,
            height: 50,
        }
    }
}

/// The initial weighting of each tile kind in the initial map generation.
///
/// These weights are non-normalized and used to determine the initial distribution of tile kinds in the map.
/// Increasing the weight of a tile kind will increase the likelihood of that tile kind appearing in the initial map.
/// Decreasing the weight of a tile kind will decrease the likelihood of that tile kind appearing in the initial map.
///
/// The weights are not normalized, so they can be any positive value,
/// or zero/omitted to indicate that the tile kind should not appear in the initial map.
#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct InitialWeights {
    weights: Vec<(TileKind, f32)>,
}

impl InitialWeights {
    /// The non-normalized weight of each state in the initial distribution used to generate the initial map.
    ///
    /// Increasing the weight of a state will increase the likelihood of that state appearing in the initial map.
    /// Decreasing the weight of a state will decrease the likelihood of that state appearing in the initial map.
    /// The weights are not normalized, so they can be any positive value,
    /// or zero to indicate that the state should not appear in the initial map.
    fn initial_distribution_weight(tile_kind: &TileKind) -> f32 {
        use TileKind::*;

        match tile_kind {
            Meadow => 1.0,
            Shrubland => 1.0,
            ShadeIntolerantForest => 0.0,
            ShadeTolerantForest => 0.0,
            // Water tiles are generated using a different mechanism
            Water => 0.0,
            Fire => 0.0,
        }
    }
}

impl Default for InitialWeights {
    fn default() -> Self {
        let mut weights = Vec::new();

        for variant in TileKind::iter() {
            weights.push((variant, Self::initial_distribution_weight(&variant)));
        }

        Self { weights }
    }
}

/// The threshold below which a tile is considered water, in the range of 0.0 to 1.0.
///
///
/// This is used in the initial map generation to determine which tiles are water based on noise values.
/// For uniformly distributed noise values, this will result in a fraction of the tiles being water
/// equal to the threshold value.
///
/// For example, a threshold of 0.4 means that 40% of the tiles will be water,
/// while a threshold of 0.2 means that 20% of the tiles will be water,
/// and a threshold of 1.0 means that all tiles will be water.
#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct WaterThreshold(f32);

impl Default for WaterThreshold {
    fn default() -> Self {
        Self(0.4)
    }
}

impl TileKind {}

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
fn determine_if_tiles_are_water(
    mut tile_query: Query<(&Position, &mut TileKind)>,
    mut rng: GlobalEntropy<WyRand>,
    water_threshold: Res<WaterThreshold>,
) {
    use noiz::prelude::*;

    // This is an example of perlin noise!
    // noiz is an incredibly powerful library for generating noise,
    // read its docs for more options!
    let mut noise = Noise::<(
        MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>,
        SNormToUNorm,
    )>::default();
    noise.set_period(5.0);
    noise.set_seed(rng.next_u32());

    for (&position, mut tile_kind) in tile_query.iter_mut() {
        let converted_position = Vec2::new(position.x as f32, position.y as f32);

        let noise_value: f32 = noise.sample(converted_position);

        // If the noise value is below a certain threshold, set the tile to water
        if noise_value < water_threshold.0 {
            *tile_kind = TileKind::Water;
        }
    }
}

// Water tiles are generated using a different mechanism, and should not be altered
#[hot]
fn randomize_land_tiles(
    mut tile_query: Query<&mut TileKind>,
    mut rng: GlobalEntropy<WyRand>,
    initial_weights: Res<InitialWeights>,
) {
    // PERF: generating multiple random choices at once is significantly faster than generating them one by one.
    for mut tile_kind in tile_query.iter_mut() {
        if *tile_kind != TileKind::Water {
            *tile_kind = initial_weights
                .weights
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
    initial_weights: Res<InitialWeights>,
    water_threshold: Res<WaterThreshold>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    if map_size.is_changed() {
        info!("Map size changed to {:?}, regenerating map", *map_size);
        next_state.set(SimState::Generate);
    }

    if initial_weights.is_changed() {
        info!("Initial weights changed, regenerating map");
        next_state.set(SimState::Generate);
    }

    if water_threshold.is_changed() {
        info!(
            "Water threshold changed to {:?}, regenerating map",
            water_threshold.0
        );
        next_state.set(SimState::Generate);
    }
}
