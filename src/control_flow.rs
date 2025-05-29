use core::time::Duration;

use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::SimState;
use crate::tile_data::Tile;

pub struct ControlFlowPlugin;

impl Plugin for ControlFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetSimulation>()
            .add_event::<PauseSimulation>()
            .add_event::<UnpauseSimulation>()
            .add_event::<StepSimulation>()
            .add_systems(
                Update,
                run_simulation
                    .run_if(in_state(SimState::Run))
                    .run_if(on_timer(Duration::from_secs(1))),
            )
            .add_systems(
                PreUpdate,
                (
                    reset_simulation_state.run_if(on_event::<ResetSimulation>),
                    pause_simulation.run_if(on_event::<PauseSimulation>),
                    unpause_simulation.run_if(on_event::<UnpauseSimulation>),
                    step_simulation.run_if(on_event::<StepSimulation>),
                ),
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

#[derive(Event)]
pub struct ResetSimulation;

fn reset_simulation_state(
    mut commands: Commands,
    tiles: Query<Entity, With<Tile>>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    info!("Resetting simulation state. Clearing all tiles and transitioning back to Generate.");

    // Remove all tiles from the map
    for tile in tiles.iter() {
        commands.entity(tile).despawn();
    }

    // Reset the next state to Generate
    next_state.set(SimState::Generate);
}

#[derive(Event)]
pub struct PauseSimulation;

#[derive(Event)]
pub struct UnpauseSimulation;

fn pause_simulation(mut next_state: ResMut<NextState<SimState>>) {
    info!("Simulation paused.");
    next_state.set(SimState::Paused);
}

fn unpause_simulation(mut next_state: ResMut<NextState<SimState>>) {
    info!("Simulation unpaused.");
    next_state.set(SimState::Run);
}

#[derive(Event)]
pub struct StepSimulation;

fn step_simulation(mut commands: Commands) {
    info!("Stepping simulation by one tick.");

    commands.run_system_cached(run_simulation);
}
