# Milestones

This project is open source! Feel free to pitch in to improve it, and tick off items on this list as you do so.

## Hello World

- [x] create project
- [x] add license(s)
- [x] write README
- [x] create the trivial `DefaultPlugins` Bevy window

## Cellular automata 101

- [x] generate an initial map
- [x] render the map
- [x] transition between states

## Control flow

- [x] pause and unpause
- [x] reset
- [x] step
- [x] modify the time step

## Dev tooling basics

- [x] set up bevy_console
- [x] set up bevy-inspector-egui
- [x] set up hotpatching
- [x] semi-optimized build settings

## Better dev tooling

- [x] show resources in the inspector
  - needed to add `#[reflect(Resource)]` and register the type
- [ ] capture logs to the console
  - tried to copy the [example](https://github.com/RichoDemus/bevy-console/blob/main/examples/capture_bevy_logs.rs), but nothing was shown

## Camera time

- [x] camera pan
- [x] camera zoom

## Fancier automata

- [x] start fires
- [x] spread fires
- [x] add water tiles

## Better camera

- [x] don't let the camera move when using the egui window
- [x] scale pan speed with zoom level
- [x] automatically scale to extents

## Better generation

- [x] use [noiz](https://docs.rs/noiz/latest/noiz/) to determine water locations
- [x] dynamically modifiable map size
- [x] use something more interesting for the noise function
- [ ] tunable map generation via a resource
- [ ] expose RNG seed

## GUI controls

- [ ] embed simulation in a viewport
- [ ] GUI for pause and unpause
- [ ] GUI for reset
- [ ] GUI for step
- [ ] GUI for modify the time step
- [ ] GUI for map presets
- [ ] hide the inspector by default, but with a button to open it
- [ ] button to open the console

## Graphs and stats

- [ ] create a stats dashboard
- [ ] create a graph that shows coverage over time

## Scenarios

- [ ] save current map state to a preset
- [ ] save and load hyperparameters
- [ ] load presets
- [ ] GUI for saving and loading scenarios

## Ooh shiny

- [ ] use sprites for each type of type of terrain
- [ ] add a map border
- [ ] visualize transition rules automatically
- [ ] add sound effects
- [ ] mouse-over tooltips for each tile

## Key bindings

- [ ] swap to `bevy_enhanced_input`
- [ ] add a keybindings menu
