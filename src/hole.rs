use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{ball::Ball, GameState};

pub struct HolePlugin;

impl Plugin for HolePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Hole>()
            .add_system(win_condition.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
pub struct Hole;

fn win_condition(
    mut collision_events: EventReader<CollisionEvent>,
    ball_query: Query<Entity, With<Ball>>,
    hole_query: Query<Entity, With<Hole>>,
    hole_mesh_query: Query<&Parent, With<Collider>>,
) {
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
                        println!("you win!!");
                    }
                }
            }
        }
    }
}
