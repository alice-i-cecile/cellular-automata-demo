# cellular-automata-demo

A proof-of-concept for using Bevy for scientific simulation, with an emphasis on rapid iteration and reusable cross-experiment tooling.

## Getting started

To get started:

1. Clone this repository.
2. Follow Bevy's [instructions](https://bevyengine.org/learn/) to set up Bevy and Rust on your machine.
3. Call `cargo run --release` to run the simulation.

## Development Tooling

This project comes with a powerful dev console, courtesy of [`bevy-console`](https://github.com/RichoDemus/bevy-console). To open it, press the `~` key on your keyboard (above the Tab key).
Enter `help` into the console to see the list of available commands.

This project includes an inspector, which can be used to examine and manipulate the state of the simulation.
This uses [`bevy-inspector-egui`](https://github.com/jakobhellermann/bevy-inspector-egui).
