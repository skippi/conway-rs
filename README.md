# conway-rs

Mindlessly simple rust implementation of Conway's Game of Life.

## Usage

On startup, the program randomly spawns cells inside of a viewing field that
matches current terminal's dimensions. This viewing field updates every 250ms
while the program is active. Press any key to exit the program.

```bash
cargo +nightly run
```

## Building

Use cargo and your choice of compiler options.

```bash
cargo +nightly build
cargo test
```
