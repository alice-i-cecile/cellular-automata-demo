use core::time::Duration;

use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::SimState;

pub struct ControlFlowPlugin;

impl Plugin for ControlFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            run_simulation
                .run_if(in_state(SimState::Run))
                .run_if(on_timer(Duration::from_secs(1))),
        );
    }
}

/// A dedicated schedule for all of our simulation logic,
/// allowing us to advance it independently of rendering or player input.
#[derive(ScheduleLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Simulation;

pub fn run_simulation(world: &mut World) {
    // Just call `world.run_schedule` whenever you feel like it, with whatever logic you please!
    world.run_schedule(Simulation);
}
