use bevy::prelude::*;
use bevy_maze::{MazeConfig, MazePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MazePlugin)
        .insert_resource(MazeConfig::new(10, 10))
        .add_startup_system(camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands, config: Res<MazeConfig>) {
    let maze_width = config.width as f32 * 32.0;
    let maze_height = config.height as f32 * 32.0;

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(maze_width / 2.0 - 16.0, maze_height / 2.0 - 16.0, 1000.0),
        ..Default::default()
    });
}
