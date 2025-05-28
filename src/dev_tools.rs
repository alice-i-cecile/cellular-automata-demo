use bevy::prelude::*;
use bevy_console::ConsolePlugin;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
    }
}
