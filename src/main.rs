mod engine;

use crate::engine::atlas::AtlasPlugin;
use crate::engine::camera::CameraPlugin;
use crate::engine::chunk::{CHUNK_HEIGHT, CHUNK_SIZE};
use crate::engine::world::WorldPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AtlasPlugin)
        .add_plugins(CameraPlugin {
            starting_pos: Vec3::new(
                CHUNK_SIZE as f32,
                CHUNK_HEIGHT as f32 + 10.,
                CHUNK_SIZE as f32,
            ),
            ..default()
        })
        .add_plugins(WorldPlugin::default())
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
