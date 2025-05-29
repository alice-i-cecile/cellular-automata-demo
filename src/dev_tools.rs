use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use clap::Parser;

use crate::control_flow::ResetSimulation;

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
        app.add_console_command::<ResetCommand, _>(reset_command);
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
