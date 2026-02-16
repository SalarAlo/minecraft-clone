mod debug;
mod engine;

use bevy::prelude::*;

use engine::atlas::AtlasPlugin;
use engine::camera::CameraPlugin;
use engine::world::chunk_meshing::ChunkMeshingPlugin;
use engine::world::streaming::StreamingPlugin;

use debug::wireframe::WireframeDebugPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AtlasPlugin)
        .add_plugins(CameraPlugin::default())
        .add_plugins(WireframeDebugPlugin::default())
        .add_plugins((ChunkMeshingPlugin, StreamingPlugin::default()))
        .insert_resource(ClearColor(Color::srgb(0.52, 0.80, 0.92)))
        .add_systems(Startup, spawn_light)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 40_000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.2, -0.9, 0.0)),
    ));
}
