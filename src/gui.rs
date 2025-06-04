//! A friendly GUI to control the simulation.
//!
//! This UI is built using bevy_ui, but similar solutions are very possible using many other UI frameworks,
//! such as bevy_egui.

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::tailwind::*,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{TextureDimension, TextureFormat, TextureUsages},
    },
};

use crate::viewport::ViewportNode;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gui);
    }
}

// We're using bundles to define our UI panel components,
// as it provides a nice simple way to configure and construct groups of related components.
#[derive(Bundle)]
struct PanelBundle {
    node: Node,
    background_color: BackgroundColor,
    border_radius: BorderRadius,
    border_color: BorderColor,
}

impl PanelBundle {
    fn from_width(width: Val) -> Self {
        Self {
            node: Node {
                width,
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                margin: UiRect::all(Val::Px(4.0)),
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor::from(GRAY_200),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            border_color: BorderColor::from(GRAY_500),
        }
    }
}

// Set up all of the GUI entities here using bevy_ui.
// We're using an exclusive system to avoid headaches around system ordering
pub fn spawn_gui(world: &mut World) {
    // Spawn a camera for the GUI
    // We're using a 3d camera here just to make it easier to distinguish it from the simulation camera.
    world.spawn(Camera3d::default());

    // Create a root UI entity
    let root = world
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(8.0)),
                align_items: AlignItems::Stretch,
                ..default()
            },
            BackgroundColor::from(GRAY_900),
        ))
        .id();

    // Spawn the left panel (controls and settings)
    let left_panel = spawn_left_panel(world);

    // Spawn the viewport
    let viewport = spawn_viewport(world);

    // Spawn the right panel (statistics)
    let right_panel = spawn_right_panel(world);

    // Assemble the UI hierarchy
    world
        .entity_mut(root)
        .add_children(&[left_panel, viewport, right_panel]);
}

/// Spawns the left panel widget, returning the root UI entity
fn spawn_left_panel(world: &mut World) -> Entity {
    let left_panel_root = world
        .spawn(PanelBundle::from_width(Val::Percent(20.0)))
        .id();

    // Text label
    let label = world.spawn((Text::new("Controls"), TextColor::BLACK)).id();

    // Assemble the hierarchy
    world.entity_mut(left_panel_root).add_child(label);

    left_panel_root
}

/// Spawns the viewport widget, retuning the root UI entity
fn spawn_viewport(world: &mut World) -> Entity {
    let viewport_root = world
        .spawn(PanelBundle::from_width(Val::Percent(60.0)))
        .id();

    // Text label
    let label = world.spawn((Text::new("Viewport"), TextColor::BLACK)).id();

    // Set up an texture for the simulation camera to render to.
    // The size of the texture will be based on the viewport's ui size.
    let mut image = Image::new_uninit(
        default(),
        TextureDimension::D2,
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::all(),
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    let mut images = world.resource_mut::<Assets<Image>>();
    let image_handle = images.add(image);

    // Spawn the simulation camera
    let simulation_camera = world
        .spawn((
            Camera2d::default(),
            Camera {
                // Render this camera before our UI camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone().into()),
                ..default()
            },
        ))
        .id();

    // Spawn the viewport node
    let viewport_node = world
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                ..default()
            },
            BackgroundColor::from(GRAY_800),
            ViewportNode::new(simulation_camera),
        ))
        .id();

    // Assemble the hierarchy
    world
        .entity_mut(viewport_root)
        .add_children(&[label, viewport_node]);

    viewport_root
}

/// Spawns the right panel widget, returning the root UI entity
fn spawn_right_panel(world: &mut World) -> Entity {
    let right_panel_root = world
        .spawn(PanelBundle::from_width(Val::Percent(20.0)))
        .id();

    // Text label
    let label = world
        .spawn((Text::new("Statistics"), TextColor::BLACK))
        .id();

    // Assemble the hierarchy
    world.entity_mut(right_panel_root).add_child(label);

    right_panel_root
}
