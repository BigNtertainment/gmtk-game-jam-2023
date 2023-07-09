use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{ball::Ball, GameState, level::LevelIndex, loading::ModelAssets};

pub struct HolePlugin;

impl Plugin for HolePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Hole>()
			.init_resource::<Won>()
            .add_system(win_condition.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
pub struct Hole;

#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct Won(pub bool);

fn win_condition(
    mut collision_events: EventReader<CollisionEvent>,
    ball_query: Query<Entity, With<Ball>>,
    hole_query: Query<Entity, With<Hole>>,
    hole_mesh_query: Query<&Parent, With<Collider>>,
	mut timer: Local<Timer>,
	mut won: ResMut<Won>,
	mut level_index: ResMut<LevelIndex>,
	mut state: ResMut<NextState<GameState>>,
	time: Res<Time>,
	models: Res<ModelAssets>,
) {
	if won.0 {
		if timer.tick(time.delta()).just_finished() {
			level_index.0 += 1;

			if level_index.0 >= models.levels.len() {
				state.set(GameState::Menu);
			} else {
				state.set(GameState::LoadLevel);
			}

			won.0 = false;
		}

		return;
	}

    if let Ok(ball) = ball_query.get_single() {
        let hole = hole_query.single();

        for collision in collision_events.iter() {
            if let CollisionEvent::Started(entity0, entity1, _) = collision {
                let other = if *entity0 == ball {
                    *entity1
                } else if *entity1 == ball {
                    *entity0
                } else {
                    continue;
                };

                if let Ok(parent) = hole_mesh_query.get(other) {
                    if parent.get() == hole {
                        won.0 = true;
						*timer = Timer::from_seconds(5., TimerMode::Once);
                    }
                }
            }
        }
    }
}
