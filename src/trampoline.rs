use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier3d::prelude::{Velocity, RapierContext};

use crate::{ball::Ball, GameState, loading::AudioAssets};

pub struct TrampolinePlugin;

impl Plugin for TrampolinePlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<Trampoline>()
			.add_system(jump.in_set(OnUpdate(GameState::Playing)));
	}
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct Trampoline {
    pub force: f32,
}

impl Default for Trampoline {
	fn default() -> Self {
		Self {
			force: 25.,
		}
	}
}

fn jump(
    mut ball_query: Query<(Entity, &mut Velocity), With<Ball>>,
    booster_query: Query<(&Trampoline, &Transform)>,
    booster_mesh_query: Query<&Parent, With<Handle<Mesh>>>,
	rapier_context: Res<RapierContext>,
	audio: Res<Audio>,
	audio_assets: Res<AudioAssets>,
) {
    if let Ok((ball, mut ball_velocity)) = ball_query.get_single_mut() {
		for contact_pair in rapier_context.contacts_with(ball) {
			let other = if contact_pair.collider1() == ball {
				contact_pair.collider2()
			} else {
				contact_pair.collider1()
			};

			if let Ok(parent) = booster_mesh_query.get(other) {
				if let Ok((trampoline, transform)) = booster_query.get(parent.get()) {
					let direction = transform.up();
					let boost_vector = direction * trampoline.force;

					ball_velocity.linvel += boost_vector;

					audio.play(audio_assets.boing.clone());
				}
			}
		}
    }
}
