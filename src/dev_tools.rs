use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use clap::Parser;

use crate::{
    SimState,
    control_flow::{
        PauseSimulation, ResetSimulation, SetSimulationTimestep, StepSimulation, UnpauseSimulation,
    },
};

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin {
                // Setting needed for bevy-inspector-egui
                enable_multipass_for_primary_context: true,
            },
            // Open the console by pressing ~
            ConsolePlugin,
            WorldInspectorPlugin::new(),
        ));

        // These commands simply send events that can be handled by the simulation logic.
        // The duplication between the various commands and events is intentional,
        // as it allows us to easily trigger the same logic via alternative means.
        app.add_console_command::<ResetCommand, _>(reset_command)
            .add_console_command::<PauseCommand, _>(pause_command)
            .add_console_command::<UnpauseCommand, _>(unpause_command)
            .add_console_command::<StepCommand, _>(step_command)
            .add_console_command::<SetTimestepCommand, _>(set_timestep_command);
    }
}

/// Resets the simulation to its initial state.
#[derive(Parser, ConsoleCommand)]
#[command(name = "reset")]
struct ResetCommand;

fn reset_command(
    mut console_command: ConsoleCommand<ResetCommand>,
    mut event_writer: EventWriter<ResetSimulation>,
) {
    if console_command.take().is_some() {
        event_writer.write(ResetSimulation);
    }
}

/// Pauses the simulation.
#[derive(Parser, ConsoleCommand)]
#[command(name = "pause")]
struct PauseCommand;

fn pause_command(
    mut console_command: ConsoleCommand<PauseCommand>,
    mut event_writer: EventWriter<PauseSimulation>,
) {
    if console_command.take().is_some() {
        event_writer.write(PauseSimulation);
    }
}

/// Unpauses the simulation.
#[derive(Parser, ConsoleCommand)]
#[command(name = "unpause")]
struct UnpauseCommand;

fn unpause_command(
    mut console_command: ConsoleCommand<UnpauseCommand>,
    mut event_writer: EventWriter<UnpauseSimulation>,
) {
    if console_command.take().is_some() {
        event_writer.write(UnpauseSimulation);
    }
}

/// Advances the simulation by one step.
#[derive(Parser, ConsoleCommand)]
#[command(name = "step")]
struct StepCommand;

fn step_command(
    mut console_command: ConsoleCommand<StepCommand>,
    mut event_writer: EventWriter<StepSimulation>,
    state: Res<State<SimState>>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    if console_command.take().is_some() {
        match state.get() {
            SimState::Paused => {
                // If the simulation is paused.
                event_writer.write(StepSimulation);
            }
            SimState::Run | SimState::Generate => {
                // If the simulation is running, we need to pause it first, then step it.
                // Otherwise it won't be perceived as a step by the user.
                next_state.set(SimState::Paused);
                event_writer.write(StepSimulation);
            }
        }
    }
}

/// Sets the simulation timestep to a specific value in milliseconds.
///
/// Lower values will make the simulation run faster, while higher values will slow it down.
/// The default value is 1000 milliseconds (1 second).
#[derive(Parser, ConsoleCommand)]
#[command(name = "set_timestep")]
struct SetTimestepCommand {
    milliseconds: u64,
}

fn set_timestep_command(
    mut console_command: ConsoleCommand<SetTimestepCommand>,
    mut event_writer: EventWriter<SetSimulationTimestep>,
) {
    if let Some(Ok(command)) = console_command.take() {
        event_writer.write(SetSimulationTimestep {
            milliseconds: command.milliseconds,
        });
    }
}
