mod engine;

use bevy::prelude::*;

use engine::atlas::AtlasPlugin;
use engine::camera::CameraPlugin;
use engine::world::generation::WorldGenerationPlugin;
use engine::world::streaming::StreamingPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AtlasPlugin)
        .add_plugins(CameraPlugin {
            starting_pos: Vec3::new(0., 100., 0.),
            ..default()
        })
        .add_plugins(WorldGenerationPlugin)
        .add_plugins(StreamingPlugin { render_distance: 8 })
        .add_systems(Startup, spawn_light)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(DirectionalLight {
            illuminance: 15_000.0,
            ..default()
        })
        .insert(Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -1.0,
            -0.8,
            0.0,
        )));
}
