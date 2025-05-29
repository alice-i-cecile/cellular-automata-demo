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

- [ ] camera pan
- [ ] camera zoom
- [ ] automatically scale to extents

## Fancier automata

- [ ] consider neighboring cells for transitions
- [ ] add fire to reset state
- [ ] add water tiles

## Better generation

- [] use [noiz](https://docs.rs/noiz/latest/noiz/) to seed biome generation
- [] dynamically modifiable map size
- [] tunable map generation
- [] scenarios and presets

## GUI controls

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

## Ooh shiny

- [ ] use sprites for each type of type of terrain
- [ ] add a map border
- [ ] visualize transition rules automatically
- [ ] add particles
- [ ] add sound effects
