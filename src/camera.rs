use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use crate::{actions::Actions, ball::Ball, util::cleanup, GameState};

// TODO: Make this a setting if there's time left.
const SENSITIVITY: f32 = 0.2;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((setup_camera, lock_cursor).in_schedule(OnEnter(GameState::Playing)))
            .add_system(look_angles.in_set(OnUpdate(GameState::Playing)))
            .add_system(cleanup::<Camera>.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component)]
struct CameraControls {
    pub radius: f32,
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(25., 7., 0.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraControls { radius: 27. });
}

fn lock_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn look_angles(
    mut query: Query<(&mut Transform, &CameraControls)>,
    target: Query<(Entity, &Transform), (With<Ball>, Without<CameraControls>)>,
    actions: Res<Actions>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok((ball, target)) = target.get_single() {
        let delta = actions.camera_movement.unwrap_or(Vec2::ZERO);
        let delta = delta * SENSITIVITY * time.delta_seconds();

        let (mut transform, camera_controls) = query.single_mut();

        let yaw = Quat::from_rotation_y(-delta.x);
        let pitch = Quat::from_rotation_x(-delta.y);

        transform.rotation = yaw * transform.rotation;
        transform.rotation = transform.rotation * pitch;

        // TODO: Clamp y rotation

        transform.rotation = transform.rotation.normalize();

        let ray_direction = -transform.forward().normalize();

        let camera_position = if let Some((_entity, toi)) = rapier_context.cast_ray(
            target.translation,
            ray_direction,
            camera_controls.radius,
            false,
            QueryFilter::new().exclude_collider(ball),
        ) {
            let hit_point = target.translation + ray_direction * (toi - 2.);

            hit_point
        } else {
            target.translation + ray_direction * camera_controls.radius
        };

        transform.translation = camera_position;
    }
}
