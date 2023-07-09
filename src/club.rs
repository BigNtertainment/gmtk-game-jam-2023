use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier3d::prelude::Velocity;

use crate::{loading::{AnimationAssets, AudioAssets}, GameState, ball::Ball};

const CLUB_FORCE: f32 = 50.;

pub struct ClubPlugin;

impl Plugin for ClubPlugin {
    fn build(&self, app: &mut App) {
		app.add_system(play_club_animation.in_set(OnUpdate(GameState::Playing)));
	}
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Club;

fn play_club_animation(
    mut players: Query<(&mut AnimationPlayer, &Transform), (With<AnimationPlayer>, Added<Club>)>,
	mut ball: Query<&mut Velocity, With<Ball>>,
    animations: Res<AnimationAssets>,
    audio_assets: Res<AudioAssets>,
	audio: Res<Audio>,
) {
	for (mut player, transform) in &mut players {
		let mut ball_velocity = ball.single_mut();

		ball_velocity.linvel += transform.forward() * CLUB_FORCE;

		audio.play(audio_assets.knock.clone());

        player.play(animations.club_hit.clone_weak());
    }
}
