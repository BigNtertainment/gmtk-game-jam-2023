use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::{loading::AudioAssets, GameState};

pub struct SoundtrackPlugin;

impl Plugin for SoundtrackPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(play_soundtrack.in_schedule(OnExit(GameState::Loading)));
	}
}

fn play_soundtrack(audio: Res<Audio>, audio_assets: Res<AudioAssets>) {
	audio.play(audio_assets.soundtrack.clone()).looped();
}
