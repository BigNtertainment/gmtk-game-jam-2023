use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{actions::Actions, GameState};

use self::ui::BallUiPlugin;

const MAX_BALL_ENERGY: f32 = 1000.;

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
    pub velocity: Velocity,
    pub collider: Collider,
}

impl Default for BallBundle {
    fn default() -> Self {
        Self {
            ball: Ball { energy: MAX_BALL_ENERGY },
            rigidbody: RigidBody::Dynamic,
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
    actions: Res<Actions>,
    time: Res<Time>,
) {
    if let Some(movement_vector) = actions.player_movement {
        let movement_vector = Vec3::new(movement_vector.x, 0., movement_vector.y);

        for (entity, mut ball) in ball_query.iter_mut() {
            if ball.energy <= 0.0 {
                continue;
            }

            let impulse = movement_vector * time.delta_seconds() * 100.0 * (ball.energy / MAX_BALL_ENERGY);

            commands.entity(entity).insert(ExternalImpulse {
                impulse,
                ..Default::default()
            });

            ball.energy -= impulse.length();
        }
    }
}
