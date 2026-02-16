use bevy::camera::Exposure;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

#[derive(Resource, Clone)]
struct CameraConfig {
    pub sensitivity: f32,
    pub speed: f32,
    pub shift_speed: f32,
    pub starting_pos: Vec3,
}

pub struct CameraPlugin {
    pub sensitivity: f32,
    pub speed: f32,
    pub shift_speed: f32,
    pub lock_cursor_on_start: bool,
    pub starting_pos: Vec3,
}

impl Default for CameraPlugin {
    fn default() -> Self {
        Self {
            sensitivity: 0.0005,
            speed: 100.0,
            shift_speed: 1000.0,
            lock_cursor_on_start: true,
            starting_pos: Vec3::ZERO,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConfig {
            sensitivity: self.sensitivity,
            speed: self.speed,
            shift_speed: self.shift_speed,
            starting_pos: self.starting_pos,
        });
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (camera_movement, camera_look, toggle_cursor));

        if self.lock_cursor_on_start {
            app.add_systems(Startup, lock_cursor);
        }
    }
}

#[derive(Component)]
pub struct CameraSettings {
    sensitivity: f32,
    speed: f32,
    shift_speed: f32,
    can_move: bool,
}

fn spawn_camera(mut commands: Commands, config: Res<CameraConfig>) {
    let transform = Transform::from_translation(config.starting_pos)
        .looking_at(Vec3::new(50., 50., 50.), Vec3::Y);

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Tonemapping::AcesFitted,
        Exposure { ev100: 12. },
        transform,
        AmbientLight {
            color: Color::WHITE,
            brightness: 3000.,
            ..default()
        },
        CameraSettings {
            sensitivity: config.sensitivity,
            speed: config.speed,
            shift_speed: config.shift_speed,
            can_move: true,
        },
    ));
}

fn camera_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    cam: Single<(&mut Transform, &CameraSettings), With<Camera3d>>,
) {
    let (mut transform, settings) = cam.into_inner();
    if !settings.can_move {
        return;
    }
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction += Vec3::from(transform.forward());
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= Vec3::from(transform.forward());
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += Vec3::from(transform.right());
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= Vec3::from(transform.right());
    }
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    let sprint = keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    let speed = if sprint {
        settings.shift_speed
    } else {
        settings.speed
    };

    transform.translation += direction * speed * time.delta_secs();
}

fn camera_look(
    mut mouse_motion: MessageReader<MouseMotion>,
    camera: Single<(&mut Transform, &CameraSettings), With<Camera3d>>,
) {
    let (mut transform, settings) = camera.into_inner();
    if !settings.can_move {
        return;
    }

    let mut delta = Vec2::ZERO;

    for ev in mouse_motion.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let yaw = Quat::from_rotation_y(-delta.x * settings.sensitivity);
    let pitch = Quat::from_rotation_x(-delta.y * settings.sensitivity);

    transform.rotation = yaw * transform.rotation;
    transform.rotation = transform.rotation * pitch;

    let (yaw_angle, pitch_angle, _) = transform.rotation.to_euler(EulerRot::YXZ);
    let max_pitch = 1.54;
    let clamped_pitch = pitch_angle.clamp(-max_pitch, max_pitch);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw_angle, clamped_pitch, 0.0);
}

fn lock_cursor(mut commands: Commands, window: Single<Entity, With<PrimaryWindow>>) {
    commands.entity(*window).insert(CursorOptions {
        grab_mode: CursorGrabMode::Locked,
        visible: false,
        ..default()
    });
}

fn toggle_cursor(
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Single<(Entity, &mut CursorOptions), With<PrimaryWindow>>,
    mut camera: Single<&mut CameraSettings>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let (entity, cursor) = window.into_inner();

        let locked = cursor.grab_mode == CursorGrabMode::Locked;

        camera.can_move = !locked;

        commands.entity(entity).insert(CursorOptions {
            grab_mode: if locked {
                CursorGrabMode::None
            } else {
                CursorGrabMode::Locked
            },
            visible: locked,
            ..default()
        });
    }
}
