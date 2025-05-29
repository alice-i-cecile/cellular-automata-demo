use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use bevy_simple_subsecond_system::hot;
use rand::seq::IndexedRandom;
use strum::IntoEnumIterator;

use crate::SimState;
use crate::tile_data::{Position, Tile, TileKind};

// PERF: these systems would all be faster as exclusive systems to avoid command overhead
pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapSize>()
            .insert_resource(MapSize {
                width: 30,
                height: 30,
            })
            .add_systems(
                OnEnter(SimState::Generate),
                (clean_up_sim_state, spawn_tiles).chain(),
            )
            .add_systems(
                Update,
                (
                    regenerate_when_settings_change,
                    finish_generation.run_if(in_state(SimState::Generate)),
                ),
            );
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
struct MapSize {
    width: i32,
    height: i32,
}

impl TileKind {
    /// The non-normalized weight of each state in the initial distribution used to generate the initial map.
    ///
    /// Increasing the weight of a state will increase the likelihood of that state appearing in the initial map.
    /// Decreasing the weight of a state will decrease the likelihood of that state appearing in the initial map.
    /// The weights are not normalized, so they can be any positive value,
    /// or zero to indicate that the state should not appear in the initial map.
    fn initial_distribution_weight(&self) -> f32 {
        use TileKind::*;

        match self {
            Meadow => 1.0,
            Shrubland => 1.0,
            ShadeIntolerantForest => 0.0,
            ShadeTolerantForest => 0.0,
            Water => 1.0,
        }
    }

    // TODO: use strum for enum iteration
    fn initial_distribution() -> Vec<(TileKind, f32)> {
        let mut vec = Vec::new();

        for variant in TileKind::iter() {
            vec.push((variant, variant.initial_distribution_weight()));
        }

        vec
    }
}

#[hot]
fn clean_up_sim_state(mut commands: Commands, query: Query<Entity, With<Tile>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[hot]
fn spawn_tiles(mut commands: Commands, map_size: Res<MapSize>, mut rng: GlobalEntropy<WyRand>) {
    let state_weights = TileKind::initial_distribution();

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
            let name = Name::new(format!("Tile ({x}, {y})"));

            commands.spawn((Tile, position, sprite, transform, succession_state, name));
        }
    }
}

fn finish_generation(mut next_state: ResMut<NextState<SimState>>) {
    info!("Map generation complete, transitioning to Run state");
    next_state.set(SimState::Run);
}

#[hot]
fn regenerate_when_settings_change(
    map_size: Res<MapSize>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    if map_size.is_changed() {
        info!("Map size changed to {:?}, regenerating map", *map_size);
        next_state.set(SimState::Generate);
    }
}
