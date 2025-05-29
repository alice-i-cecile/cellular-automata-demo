use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::Entropy;
use bevy_simple_subsecond_system::hot;
use rand::Rng;
use rand::seq::IndexedRandom;
use strum_macros::EnumIter;

use crate::control_flow::Simulation;

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TileKind>()
            .add_systems(Simulation, (undisturbed_succession, start_fires).chain());
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
    mut succession_query: Query<&mut TileKind>,
) {
    for mut tile_kind in succession_query.iter_mut() {
        let new_kind = tile_kind.transition(&mut rng);
        tile_kind.set_if_neq(new_kind);
    }
}

#[hot]
fn start_fires(mut tile_query: Query<&mut TileKind>, mut rng: GlobalEntropy<WyRand>) {
    for mut tile in tile_query.iter_mut() {
        let fire_roll = rng.random_range(0.0..1.0);
        if fire_roll < tile.fire_susceptibility() {
            // If the tile is susceptible to fire, set it to Fire state
            tile.set_if_neq(TileKind::Fire);
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
                vec![(Fire, 1.0), (Meadow, 0.5), (Shrubland, 0.5)]
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

        match self {
            Meadow => 0.005,
            Shrubland => 0.01,
            ShadeIntolerantForest => 0.01,
            ShadeTolerantForest => 0.02,
            Water => 0.0, // Water cannot catch fire
            Fire => 0.0,  // Fire is already burning
        }
    }
}
