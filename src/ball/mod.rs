use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::{actions::Actions, hole::Won, loading::AudioAssets, GameState};

use self::ui::BallUiPlugin;

const MAX_BALL_ENERGY: f32 = 50.;

mod ui;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ball>()
            .add_plugin(BallUiPlugin)
            .add_systems(
                (ball_movement, lose_velocity, lose_condition, play_knock_sound).in_set(OnUpdate(GameState::Playing)),
            );
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
    pub active_events: ActiveEvents,
    pub velocity: Velocity,
    pub friction: Friction,
    pub collider: Collider,
}

impl Default for BallBundle {
    fn default() -> Self {
        Self {
            ball: Ball {
                energy: MAX_BALL_ENERGY,
            },
            rigidbody: RigidBody::Dynamic,
            active_events: ActiveEvents::COLLISION_EVENTS,
            velocity: Velocity {
                linvel: Vec3::new(0., 0., 0.),
                angvel: Vec3::new(0., 0., 0.),
            },
            friction: Friction::new(1.),
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

            let impulse = movement_vector * time.delta_seconds() * 10.;

            commands.entity(entity).insert(ExternalImpulse {
                impulse,
                ..Default::default()
            });

            ball.energy -= impulse.length();
        }
    }
}

fn lose_velocity(mut query: Query<&mut Velocity, With<Ball>>, time: Res<Time>) {
    if let Ok(mut velocity) = query.get_single_mut() {
        let deacceleration = 0.5 * time.delta_seconds();

        velocity.linvel *= (1. - deacceleration).max(0.);
    }
}

fn lose_condition(
    query: Query<(&Ball, &Velocity, &Transform)>,
    won: Res<Won>,
    mut state: ResMut<NextState<GameState>>,
) {
    if won.0 {
        return;
    }

    if let Ok((ball, velocity, transform)) = query.get_single() {
        if transform.translation.y <= -10.
            || (ball.energy <= 0. && velocity.linvel.length() <= 0.05)
        {
            state.set(GameState::LoadLevel);
        }
    }
}

fn play_knock_sound(
    ball_query: Query<Entity, With<Ball>>,
    rapier_context: Res<RapierContext>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    if let Ok(ball) = ball_query.get_single() {
        for contact_pair in rapier_context.contacts_with(ball) {
            for manifold in contact_pair.manifolds() {
                for contact_point in manifold.points() {
                    if contact_point.impulse().abs() > 1. {
                        audio
                            .play(audio_assets.knock.clone())
                            .with_playback_rate(0.9 + rand::thread_rng().gen::<f64>() / 5.);

                        return;
                    }
                }
            }
        }
    }
}
