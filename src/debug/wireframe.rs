use bevy::input::keyboard::KeyCode;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;

pub struct WireframeDebugPlugin {
    toggle_key: KeyCode,
}

#[derive(Resource)]
struct WireframeDebugConfig {
    toggle_key: KeyCode,
}

impl Default for WireframeDebugPlugin {
    fn default() -> Self {
        Self {
            toggle_key: KeyCode::KeyP,
        }
    }
}

impl Plugin for WireframeDebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WireframeDebugConfig {
            toggle_key: self.toggle_key,
        })
        .insert_resource(WireframeConfig {
            global: false,
            ..default()
        })
        .add_plugins(WireframePlugin::default())
        .add_systems(Update, toggle_wireframe);
    }
}

fn toggle_wireframe(
    conf: Res<WireframeDebugConfig>,
    keys: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<WireframeConfig>,
) {
    if keys.just_pressed(conf.toggle_key) {
        config.global = !config.global;
        info!("Wireframe: {}", config.global);
    }
}
