use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_event::<BurstActions>()
            .add_systems((set_movement_actions, set_burst_actions).in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub camera_movement: Option<Vec2>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BurstActions {
    Reset
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }

    if mouse_motion.is_empty() {
        actions.camera_movement = None;
    } else {
        actions.camera_movement = Some({
            let mut camera_movement = Vec2::ZERO;

            for motion in mouse_motion.iter() {
                camera_movement += motion.delta;
            }

            camera_movement
        });
    }
}

fn set_burst_actions(
    keyboard_input: Res<Input<KeyCode>>,
    mut burst_actions: EventWriter<BurstActions>,
) {
    if GameControl::pressed(&GameControl::Reset, &keyboard_input) {
        burst_actions.send(BurstActions::Reset);
    }
}
