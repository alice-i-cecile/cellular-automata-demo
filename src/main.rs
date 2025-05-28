use std::hash::Hash;

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_prng::WyRand;
use bevy_rand::{global::GlobalEntropy, plugin::EntropyPlugin, prelude::Entropy};
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .insert_resource(MapSize {
            width: 20,
            height: 20,
        })
        .init_resource::<TileImages>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_tiles)
        .add_systems(Update, update_tile_graphics)
        .run();
}

#[derive(Resource)]
struct MapSize {
    width: i32,
    height: i32,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const PIXELS_PER_TILE: f32 = 32.0;

    fn to_transform(&self) -> Transform {
        Transform::from_xyz(
            self.x as f32 * Self::PIXELS_PER_TILE,
            self.y as f32 * Self::PIXELS_PER_TILE,
            0.0,
        )
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_tiles(mut commands: Commands, map_size: Res<MapSize>, mut rng: GlobalEntropy<WyRand>) {
    let state_weights = SuccessionState::initial_distribution();

    // PERF: we could speed this up by using spawn_batch
    // PERF: generating multiple random choices at once is significantly faster than generating them one by one.
    for x in 0..map_size.width {
        for y in 0..map_size.height {
            let position = Position { x, y };
            let transform = position.to_transform();
            let succession_state = state_weights
                .choose_weighted(&mut rng, |item| item.1)
                .unwrap()
                .0;
            let sprite = Sprite {
                color: succession_state.color(),
                custom_size: Some(Vec2::splat(Position::PIXELS_PER_TILE)),
                ..Default::default()
            };

            commands.spawn((position, sprite, transform, succession_state));
        }
    }
}

#[derive(Resource, Deref)]
struct TileImages {
    colors: HashMap<SuccessionState, Color>,
}

impl FromWorld for TileImages {
    fn from_world(_world: &mut World) -> Self {
        let mut colors = HashMap::new();

        for variant in SuccessionState::iter() {
            colors.insert(variant, variant.color());
        }

        Self { colors }
    }
}

fn update_tile_graphics(
    mut tile_query: Query<(&mut Sprite, &SuccessionState), Changed<SuccessionState>>,
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

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy, EnumIter)]
#[require(Sprite)]
enum SuccessionState {
    Meadow,
    Shrubland,
    ShadeIntolerantForest,
    ShadeTolerantForest,
}

impl SuccessionState {
    /// The color associated with this state.
    ///
    /// This is used to determine the color of the tile in the map.
    fn color(&self) -> Color {
        use SuccessionState::*;

        match self {
            Meadow => Color::hsl(84., 0.7, 0.8),
            Shrubland => Color::hsl(84., 0.5, 0.5),
            ShadeIntolerantForest => Color::hsl(84., 0.3, 0.5),
            ShadeTolerantForest => Color::hsl(84., 0.2, 0.2),
        }
    }

    /// The non-normalized weight of each state in the initial distribution used to generate the initial map.
    ///
    /// Increasing the weight of a state will increase the likelihood of that state appearing in the initial map.
    /// Decreasing the weight of a state will decrease the likelihood of that state appearing in the initial map.
    /// The weights are not normalized, so they can be any positive value,
    /// or zero to indicate that the state should not appear in the initial map.
    fn initial_distribution_weight(&self) -> f32 {
        match self {
            SuccessionState::Meadow => 1.0,
            SuccessionState::Shrubland => 0.0,
            SuccessionState::ShadeIntolerantForest => 0.0,
            SuccessionState::ShadeTolerantForest => 0.0,
        }
    }

    // TODO: use strum for enum iteration
    fn initial_distribution() -> Vec<(SuccessionState, f32)> {
        let mut vec = Vec::new();

        for variant in SuccessionState::iter() {
            vec.push((variant, variant.initial_distribution_weight()));
        }

        vec
    }

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
