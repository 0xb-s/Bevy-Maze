use crate::{Maze, MazeConfig};
use bevy::prelude::*;

/// Setup system to generate and render the maze.
pub fn setup_maze(mut commands: Commands, config: Res<MazeConfig>) {
    let mut maze = Maze::new(config.width, config.height);
    maze.generate();

    let cell_size = 32.0; 
    let wall_thickness = 4.0;

    for cell in maze.grid.iter() {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::new(cell_size, cell_size)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                cell.x as f32 * cell_size,
                cell.y as f32 * cell_size,
                0.0,
            ),
            ..Default::default()
        });

        if cell.walls[0] {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.2, 0.2),
                    custom_size: Some(Vec2::new(cell_size, wall_thickness)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    cell.x as f32 * cell_size,
                    cell.y as f32 * cell_size + cell_size / 2.0,
                    1.0,
                ),
                ..Default::default()
            });
        }
        if cell.walls[1] {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.2, 0.2),
                    custom_size: Some(Vec2::new(wall_thickness, cell_size)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    cell.x as f32 * cell_size + cell_size / 2.0,
                    cell.y as f32 * cell_size,
                    1.0,
                ),
                ..Default::default()
            });
        }
    }
}
