use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{actions::Actions, GameState};

use self::ui::BallUiPlugin;

const MAX_BALL_ENERGY: f32 = 100.;

mod ui;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ball>()
            .add_plugin(BallUiPlugin)
            .add_system(ball_movement.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Ball {
    energy: f32,
}

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub rigidbody: RigidBody,
    pub ccd: Ccd,
    pub velocity: Velocity,
    pub collider: Collider,
}

impl Default for BallBundle {
    fn default() -> Self {
        Self {
            ball: Ball {
                energy: MAX_BALL_ENERGY,
            },
            rigidbody: RigidBody::Dynamic,
            ccd: Ccd::enabled(),
            velocity: Velocity {
                linvel: Vec3::new(0., 0., 0.),
                angvel: Vec3::new(0., 0., 0.),
            },
            collider: Collider::ball(1.),
        }
    }
}

fn ball_movement(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Ball)>,
    camera_query: Query<&Transform, With<Camera>>,
    actions: Res<Actions>,
    time: Res<Time>,
) {
    if let Some(movement_vector) = actions.player_movement {
        let camera_transform = camera_query.single();

        let forward = (camera_transform.forward() * Vec3::new(1., 0., 1.)).normalize();
        let right = (camera_transform.right() * Vec3::new(1., 0., 1.)).normalize();

        let movement_vector = (right * movement_vector.x + forward * movement_vector.y).normalize();

        for (entity, mut ball) in ball_query.iter_mut() {
            if ball.energy <= 0.0 {
                continue;
            }

            let impulse =
                movement_vector * time.delta_seconds() * 10. * (ball.energy / MAX_BALL_ENERGY);

            commands.entity(entity).insert(ExternalImpulse {
                impulse,
                ..Default::default()
            });

            ball.energy -= impulse.length();
        }
    }
}
