# cellular-automata-demo

A proof-of-concept for using Bevy for scientific simulation, with an emphasis on rapid iteration and reusable cross-experiment tooling.

## Getting started

1. Clone this repository.
2. Follow Bevy's [instructions](https://bevyengine.org/learn/) to set up Bevy and Rust on your machine.
3. Call `cargo run` to run the simulation.

## Development Tooling

This project comes with a powerful dev console, courtesy of [`bevy-console`](https://github.com/RichoDemus/bevy-console). To open it, press the `~` key on your keyboard (above the Tab key).
Enter `help` into the console to see the list of available commands.

This project includes an inspector, which can be used to examine and manipulate the state of the simulation.
This uses [`bevy-inspector-egui`](https://github.com/jakobhellermann/bevy-inspector-egui).

This project has hotpatching enabled, allowing you to change code without restarting the simulation.
Please refer to [`bevy_simple_subsecond_system`](https://github.com/TheBevyFlock/bevy_simple_subsecond_system) for instructions on the initial setup.

Once you have hotpatching working, annotate any system you want to hotpatch with `#[hot]`, and then run your application using `dx serve --hotpatch`.
