use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::Entropy;
use rand::seq::IndexedRandom;

use crate::control_flow::Simulation;
use crate::tile_data::SuccessionState;

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Simulation, apply_transition);
    }
}

fn apply_transition(
    mut rng: GlobalEntropy<WyRand>,
    mut succession_query: Query<&mut SuccessionState>,
) {
    for mut succession_state in succession_query.iter_mut() {
        let new_state = succession_state.transition(&mut rng);
        succession_state.set_if_neq(new_state);
    }
}

impl SuccessionState {
    /// The probability of transitioning to each other state from this state.
    ///
    /// These are non-normalized: higher values will increase the likelihood of transitioning to that state,
    /// while lower values will decrease the likelihood of transitioning to that state.
    ///
    /// Missing entries in the map indicate that the state cannot transition to that state,
    /// and are equivalent to a transition probability of zero.
    fn transition_probabilities(&self) -> Vec<(SuccessionState, f32)> {
        use SuccessionState::*;

        match self {
            SuccessionState::Meadow => {
                vec![(Meadow, 1.0), (Shrubland, 0.5)]
            }
            SuccessionState::Shrubland => {
                vec![(Shrubland, 1.0), (ShadeIntolerantForest, 0.5)]
            }
            SuccessionState::ShadeIntolerantForest => {
                vec![(ShadeIntolerantForest, 1.0), (ShadeTolerantForest, 0.5)]
            }
            SuccessionState::ShadeTolerantForest => {
                vec![(ShadeTolerantForest, 1.0)]
            }
        }
    }

    fn transition(&self, mut rng: &mut Entropy<WyRand>) -> Self {
        let transition_probabilities = self.transition_probabilities();
        transition_probabilities
            .choose_weighted(&mut rng, |item| item.1)
            .unwrap()
            .0
    }
}
