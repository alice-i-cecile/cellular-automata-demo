use bevy::{platform::collections::HashMap, prelude::*};
use strum::IntoEnumIterator;

use crate::control_flow::run_simulation;
use crate::tile_data::TileKind;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileImages>()
            .add_systems(Update, update_tile_graphics.after(run_simulation));
    }
}

#[derive(Resource, Deref)]
struct TileImages {
    colors: HashMap<TileKind, Color>,
}

impl FromWorld for TileImages {
    fn from_world(_world: &mut World) -> Self {
        let mut colors = HashMap::new();

        for variant in TileKind::iter() {
            colors.insert(variant, variant.color());
        }

        Self { colors }
    }
}

fn update_tile_graphics(
    mut tile_query: Query<(&mut Sprite, &TileKind), Changed<TileKind>>,
    tile_materials: ResMut<TileImages>,
) {
    for (mut sprite, succession_state) in tile_query.iter_mut() {
        let Some(new_color) = tile_materials.get(succession_state) else {
            warn_once!("Tile graphics not found for {succession_state:?}");

            continue;
        };

        sprite.color = new_color.clone();
    }
}

impl TileKind {
    /// The color associated with this state.
    ///
    /// This is used to determine the color of the tile in the map.
    pub fn color(&self) -> Color {
        use TileKind::*;

        match self {
            Meadow => Color::hsl(84., 0.7, 0.8),
            Shrubland => Color::hsl(84., 0.5, 0.5),
            ShadeIntolerantForest => Color::hsl(84., 0.3, 0.5),
            ShadeTolerantForest => Color::hsl(84., 0.2, 0.2),
        }
    }
}
