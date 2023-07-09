use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use crate::{actions::Actions, ball::Ball, util::cleanup, GameState};

// TODO: Make this a setting if there's time left.
const SENSITIVITY: f32 = 0.2;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((setup_camera, lock_cursor).in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (update_camera, camera_control)
                    .chain()
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_systems(
                (cleanup::<Camera>, unlock_cursor).in_schedule(OnExit(GameState::Playing)),
            );
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component)]
struct CameraControls {
    pub radius: f32,
    pub pitch: f32,
    pub yaw: f32,
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(CameraControls {
            yaw: 0.,
            pitch: -0.5,
            radius: 27.,
        });
}

fn lock_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn unlock_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}

fn camera_control(mut query: Query<&mut CameraControls>, actions: Res<Actions>, time: Res<Time>) {
    let delta = actions.camera_movement.unwrap_or(Vec2::ZERO);
    let delta = delta * SENSITIVITY * time.delta_seconds();

    let mut camera_controls = query.single_mut();

    camera_controls.yaw -= delta.x;
    camera_controls.pitch -= delta.y;

    camera_controls.yaw %= 360.;
    camera_controls.pitch = camera_controls.pitch.clamp(-1.1, 0.2);
}

fn update_camera(
    mut query: Query<(&mut Transform, &CameraControls)>,
    target: Query<(Entity, &Transform), (With<Ball>, Without<CameraControls>)>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok((ball, target)) = target.get_single() {
        let (mut transform, camera_controls) = query.single_mut();

        transform.rotation = Quat::IDENTITY;

        transform.rotate_x(camera_controls.pitch);
        transform.rotate_y(camera_controls.yaw);

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
