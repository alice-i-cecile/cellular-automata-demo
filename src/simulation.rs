//! The simulation logic for the demo simulation.
//!
//! In this case, we're doing a simple grid-based
//! forest succession simulation with fire spread mechanics.
//!
//! All of this can be easily ripped out and replaced with your own simulation logic!

use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::Entropy;
use bevy_simple_subsecond_system::hot;
use rand::Rng;
use rand::seq::IndexedRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::control_flow::Simulation;
use crate::spatial_index::{Position, TileIndex};

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TileKind>()
            .init_resource::<FireSpread>()
            .register_type::<FireSpread>()
            .init_resource::<FireSusceptibility>()
            .register_type::<FireSusceptibility>()
            .init_resource::<TransitionProbabilities>()
            .register_type::<TransitionProbabilities>()
            .add_systems(
                Simulation,
                // Using .chain() is a simple but effective way to carefully control system ordering for simulations
                // In more complex simulations, consider using a vec of systems rather than a Schedule
                (spread_fires, undisturbed_succession, start_fires).chain(),
            );
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct FireSpread {
    /// The ratio of fire spread probability to the base fire susceptibility.
    /// This multiplier can be adjusted to control how quickly fire spreads.
    /// Generally this value should be significantly larger than 1.
    spread_multiplier: f64,
}

impl Default for FireSpread {
    fn default() -> Self {
        Self {
            spread_multiplier: 1e3,
        }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct FireSusceptibility {
    /// The base fire susceptibility of the tile.
    /// This is a multiplier applied to each tile's fire susceptibility,
    /// and will scale all fire susceptibility values at once.
    base_susceptibility: f64,
    /// The relative, unnormalized fire susceptibility of each tile kind.
    tile_susceptibility: HashMap<TileKind, f64>,
}

impl FireSusceptibility {
    /// Returns the fire susceptibility of a tile kind,
    /// scaled by the base susceptibility.
    ///
    /// If the tile kind is not found, it returns 0.0.
    pub fn get(&self, tile_kind: &TileKind) -> f64 {
        self.tile_susceptibility
            .get(tile_kind)
            .cloned()
            .unwrap_or(0.0)
            * self.base_susceptibility
    }
}

impl Default for FireSusceptibility {
    fn default() -> Self {
        let mut tile_susceptibility = HashMap::new();
        tile_susceptibility.insert(TileKind::Meadow, 0.01);
        tile_susceptibility.insert(TileKind::Shrubland, 0.2);
        tile_susceptibility.insert(TileKind::ShadeIntolerantForest, 0.5);
        tile_susceptibility.insert(TileKind::ShadeTolerantForest, 1.0);
        tile_susceptibility.insert(TileKind::Water, 0.0); // Water cannot catch fire
        tile_susceptibility.insert(TileKind::Fire, 0.0); // Fire is already burning

        Self {
            base_susceptibility: 1e-3,
            tile_susceptibility,
        }
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

#[hot]
fn undisturbed_succession(
    mut rng: GlobalEntropy<WyRand>,
    transition_probabilities: Res<TransitionProbabilities>,
    mut succession_query: Query<&mut TileKind>,
) {
    for mut tile_kind in succession_query.iter_mut() {
        if let Some(new_kind) = transition_probabilities.choose_transition(&*tile_kind, &mut rng) {
            *tile_kind = new_kind;
        }
    }
}

#[hot]
fn start_fires(
    mut tile_query: Query<&mut TileKind>,
    fire_susceptibility: Res<FireSusceptibility>,
    mut rng: GlobalEntropy<WyRand>,
) {
    for mut tile_kind in tile_query.iter_mut() {
        let fire_roll = rng.random_range(0.0..1.0);
        if fire_roll < fire_susceptibility.get(&*tile_kind) {
            // If the tile rolled a new fire, set it to Fire state
            tile_kind.set_if_neq(TileKind::Fire);
        }
    }
}

#[hot]
fn spread_fires(
    tile_query: Query<(&TileKind, &Position)>,
    fire_susceptibility: Res<FireSusceptibility>,
    fire_spread: Res<FireSpread>,
    mut rng: GlobalEntropy<WyRand>,
    tile_index: Res<TileIndex>,
    mut commands: Commands,
) {
    for (tile, position) in tile_query.iter() {
        if *tile == TileKind::Fire {
            for neighbors in position.cardinal_neighbors() {
                if let Some(neighbor_entity) = tile_index.get(&neighbors) {
                    if let Ok((neighbor_kind, _neighbor_position)) = tile_query.get(neighbor_entity)
                    {
                        // Check if the neighboring tile can catch fire
                        // PERF: like usual, generating random numbers in batch is much faster
                        let fire_roll = rng.random_range(0.0..1.0);
                        if fire_roll
                            < fire_susceptibility.get(neighbor_kind) * fire_spread.spread_multiplier
                        {
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

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct TransitionProbabilities {
    /// The probability of transitioning to each other state from this state in the absence of another disturbance.
    ///
    /// The key is the current state, and the value is a vector of tuples,
    /// where each tuple contains a possible transition state and its associated unnormalized probability.
    probabilities: HashMap<TileKind, Vec<(TileKind, f32)>>,
}

impl TransitionProbabilities {
    fn get(&self, tile_kind: &TileKind) -> Option<&Vec<(TileKind, f32)>> {
        self.probabilities.get(tile_kind)
    }

    fn choose_transition(
        &self,
        tile_kind: &TileKind,
        mut rng: &mut Entropy<WyRand>,
    ) -> Option<TileKind> {
        let weighted_options = self.get(tile_kind)?;
        let selection = weighted_options
            .choose_weighted(&mut rng, |item| item.1)
            .ok()?;

        Some(selection.0)
    }
}

impl Default for TransitionProbabilities {
    fn default() -> Self {
        let mut probabilities = HashMap::new();
        for tile_kind in TileKind::iter() {
            probabilities.insert(tile_kind, tile_kind.undisturbed_transition_probabilities());
        }
        Self { probabilities }
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
}
