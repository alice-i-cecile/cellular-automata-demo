//! The simulation logic for the demo simulation.
//!
//! In this case, we're doing a simple grid-based
//! forest succession simulation with fire spread mechanics.
//!
//! All of this can be easily ripped out and replaced with your own simulation logic!

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::Entropy;
// use bevy_simple_subsecond_system::hot;
use rand::Rng;
use rand::seq::IndexedRandom;
use strum_macros::EnumIter;

use crate::control_flow::Simulation;
use crate::spatial_index::{Position, TileIndex};

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TileKind>().add_systems(
            Simulation,
            // Using .chain() is a simple but effective way to carefully control system ordering for simulations
            // In more complex simulations, consider using a vec of systems rather than a Schedule
            (spread_fires, undisturbed_succession, start_fires).chain(),
        );
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

// #[hot]
fn undisturbed_succession(
    mut rng: GlobalEntropy<WyRand>,
    mut succession_query: Query<&mut TileKind>,
) {
    for mut tile_kind in succession_query.iter_mut() {
        let new_kind = tile_kind.transition(&mut rng);
        tile_kind.set_if_neq(new_kind);
    }
}

// #[hot]
fn start_fires(mut tile_query: Query<&mut TileKind>, mut rng: GlobalEntropy<WyRand>) {
    for mut tile in tile_query.iter_mut() {
        let fire_roll = rng.random_range(0.0..1.0);
        if fire_roll < tile.fire_susceptibility() {
            // If the tile is susceptible to fire, set it to Fire state
            tile.set_if_neq(TileKind::Fire);
        }
    }
}

// #[hot]
fn spread_fires(
    tile_query: Query<(&TileKind, &Position)>,
    mut rng: GlobalEntropy<WyRand>,
    tile_index: Res<TileIndex>,
    mut commands: Commands,
) {
    // The ratio of fire spread probability to the base fire susceptibility.
    // This multiplier can be adjusted to control how quickly fire spreads.
    // Generally this value should be significantly larger than 1.
    const SPREAD_MULTIPLIER: f64 = 1e3;

    for (tile, position) in tile_query.iter() {
        if *tile == TileKind::Fire {
            for neighbors in position.cardinal_neighbors() {
                if let Some(neighbor_entity) = tile_index.get(&neighbors) {
                    if let Ok((neighbor_kind, _neighbor_position)) = tile_query.get(neighbor_entity)
                    {
                        // Check if the neighboring tile can catch fire
                        // PERF: like usual, generating random numbers in batch is much faster
                        let fire_roll = rng.random_range(0.0..1.0);
                        if fire_roll < neighbor_kind.fire_susceptibility() * SPREAD_MULTIPLIER {
                            // If the roll passes, set the neighboring tile to Fire state
                            // We use `Commands` here to avoid pain with mutable borrow rules,
                            // but also to ensure that the iteration order of `tile_query` does not matter.
                            commands.entity(neighbor_entity).insert(TileKind::Fire);
                        }
                    }
                }
            }
        }
    }
}

impl TileKind {
    /// The probability of transitioning to each other state from this state in the absence of another disturbance.
    ///
    /// These are non-normalized: higher values will increase the likelihood of transitioning to that state,
    /// while lower values will decrease the likelihood of transitioning to that state.
    ///
    /// Missing entries in the map indicate that the state cannot transition to that state,
    /// and are equivalent to a transition probability of zero.
    fn undisturbed_transition_probabilities(&self) -> Vec<(TileKind, f32)> {
        use TileKind::*;

        match self {
            TileKind::Meadow => {
                vec![(Meadow, 1.0), (Shrubland, 0.5)]
            }
            TileKind::Shrubland => {
                vec![(Shrubland, 1.0), (ShadeIntolerantForest, 0.5)]
            }
            TileKind::ShadeIntolerantForest => {
                vec![(ShadeIntolerantForest, 1.0), (ShadeTolerantForest, 0.5)]
            }
            TileKind::ShadeTolerantForest => {
                vec![(ShadeTolerantForest, 1.0)]
            }
            TileKind::Water => {
                vec![(Water, 1.0)]
            }
            // These values control how long fire will burn before transitioning to another state.
            TileKind::Fire => {
                vec![(Fire, 0.5), (Meadow, 0.5), (Shrubland, 0.2)]
            }
        }
    }

    fn transition(&self, mut rng: &mut Entropy<WyRand>) -> Self {
        let transition_probabilities = self.undisturbed_transition_probabilities();
        transition_probabilities
            .choose_weighted(&mut rng, |item| item.1)
            .unwrap()
            .0
    }

    /// The probability of tiles of this state catching fire during a single simulation step.
    ///
    /// A value of 0.0 means the tile cannot catch fire,
    /// a value of 1.0 means the tile will always catch fire.
    ///
    /// Also scales susceptibility to fire spread from neighboring tiles.
    fn fire_susceptibility(&self) -> f64 {
        use TileKind::*;

        // Splitting this out into a constant makes it easier to tweak and reason about
        // the relative fire susceptibility of different tile kinds.
        const GLOBAL_FIRE_SUSCEPTIBILITY_MULTIPLIER: f64 = 1e-3;

        let base_value = match self {
            Meadow => 0.01,
            Shrubland => 0.2,
            ShadeIntolerantForest => 0.5,
            ShadeTolerantForest => 1.0,
            Water => 0.0, // Water cannot catch fire
            Fire => 0.0,  // Fire is already burning
        };

        base_value * GLOBAL_FIRE_SUSCEPTIBILITY_MULTIPLIER
    }
}
