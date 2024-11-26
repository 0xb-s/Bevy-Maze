# Maze Grid Library

A Bevy-compatible crate for generating and visualizing mazes in 2D. It uses the recursive backtracking algorithm to generate perfect mazes and renders them dynamically.

## Usage

```rust
use bevy::prelude::*;
use maze_grid::{MazePlugin, MazeConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MazePlugin)
        .insert_resource(MazeConfig::new(10, 10))
        .run();
}
```

## Features
- Recursive backtracking for perfect maze generation.
- Configurable maze size using `MazeConfig`.
- Dynamic rendering of walls and cells with Bevy sprites.

## License
This project is licensed under the MIT License.
