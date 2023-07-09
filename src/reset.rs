use bevy::prelude::*;

use crate::{GameState, actions::BurstActions};

pub struct ResetPlugin;

impl Plugin for ResetPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(reset.in_set(OnUpdate(GameState::Playing)));
	}
}

fn reset(mut burst_actions: EventReader<BurstActions>, mut state: ResMut<NextState<GameState>>) {
	for action in burst_actions.iter() {
		if *action == BurstActions::Reset {
			state.set(GameState::LoadLevel);
		}
	}
}
