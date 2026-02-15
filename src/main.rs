mod engine;

use bevy::prelude::*;

use engine::atlas::AtlasPlugin;
use engine::camera::CameraPlugin;
use engine::world::chunk_meshing::ChunkMeshingPlugin;
use engine::world::streaming::StreamingPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AtlasPlugin)
        .add_plugins(CameraPlugin {
            starting_pos: Vec3::new(0., 100., 0.),
            ..default()
        })
        .add_plugins(ChunkMeshingPlugin)
        .add_plugins(StreamingPlugin {
            render_distance: 12,
        })
        .add_systems(Startup, spawn_light)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(DirectionalLight {
            illuminance: 20_000.0,
            ..default()
        })
        .insert(Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -0.5,
            -0.5,
            0.5,
        )));
}
