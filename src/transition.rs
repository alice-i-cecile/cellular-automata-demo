use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::Entropy;
use rand::seq::IndexedRandom;

use crate::SuccessionState;

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, _app: &mut App) {}
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
