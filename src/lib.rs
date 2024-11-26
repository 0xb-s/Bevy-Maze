//! # Maze Grid
//! A Bevy-compatible crate for generating and visualizing mazes.

mod maze;
mod systems;

pub use crate::maze::*;
pub use crate::systems::*;

use bevy::prelude::*;

/// Configuration for the maze.
#[derive(Resource, Default)]
pub struct MazeConfig {
    pub width: usize,
    pub height: usize,
}

impl MazeConfig {
    /// Creates a new maze configuration.
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

/// A plugin for generating and rendering mazes in Bevy.
pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeConfig>()
            .add_startup_system(systems::setup_maze);
    }
}
