use bevy::prelude::*;
use bevy_rapier3d::prelude::{Velocity, RapierContext};

use crate::{ball::Ball, GameState};

pub struct BoosterPlugin;

impl Plugin for BoosterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Booster>()
			.add_system(boost.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct Booster {
    pub force: f32,
}

impl Default for Booster {
	fn default() -> Self {
		Self {
			force: 50.,
		}
	}
}

fn boost(
    mut ball_query: Query<(Entity, &mut Velocity), With<Ball>>,
    booster_query: Query<(&Booster, &Transform)>,
    booster_mesh_query: Query<&Parent, With<Handle<Mesh>>>,
    time: Res<Time>,
	rapier_context: Res<RapierContext>,
) {
    if let Ok((ball, mut ball_velocity)) = ball_query.get_single_mut() {
		for contact_pair in rapier_context.contacts_with(ball) {
			let other = if contact_pair.collider1() == ball {
				contact_pair.collider2()
			} else {
				contact_pair.collider1()
			};

			if let Ok(parent) = booster_mesh_query.get(other) {
				if let Ok((booster, transform)) = booster_query.get(parent.get()) {
					let direction = -transform.forward();
					let boost_vector = direction * time.delta_seconds() * booster.force;

					ball_velocity.linvel += boost_vector;
				}
			}
		}
    }
}
