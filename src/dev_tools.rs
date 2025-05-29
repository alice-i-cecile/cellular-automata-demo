use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use clap::Parser;

use crate::{SimState, tile_data::Tile};

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

        // Resetting
        app.add_console_command::<ResetCommand, _>(reset_simulation_state);
    }
}

/// Resets the simulation to its initial state.
#[derive(Parser, ConsoleCommand)]
#[command(name = "reset")]
struct ResetCommand;

fn reset_simulation_state(
    mut console_command: ConsoleCommand<ResetCommand>,
    mut commands: Commands,
    tiles: Query<Entity, With<Tile>>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    if console_command.take().is_none() {
        // If the command was not invoked, do nothing
        return;
    }

    info!("Resetting simulation state. Clearing all tiles and transitioning back to Generate.");

    // Remove all tiles from the map
    for tile in tiles.iter() {
        commands.entity(tile).despawn();
    }

    // Reset the next state to Generate
    next_state.set(SimState::Generate);
}
