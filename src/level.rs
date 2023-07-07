use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

use crate::{ball::BallBundle, loading::ModelAssets, GameState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HookPlugin)
            .add_system(load_level.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn load_level(mut commands: Commands, models: Res<ModelAssets>) {
    commands
        .spawn(HookedSceneBundle {
            scene: SceneBundle {
                scene: models.golfball.clone(),
                ..default()
            },
            hook: SceneHook::new(|entity, commands| {
                match entity.get::<Name>().map(|name| name.as_str()) {
                    Some("Icosphere") => {
                        if let Some(_mesh) = entity.get::<Handle<Mesh>>() {
                            commands.insert(BallBundle::default());
                        }
                    }
                    _ => {}
                }
            }),
        })
        .insert(Name::new("level"));

    commands
        .spawn(TransformBundle {
            local: Transform::from_xyz(0., -5., 0.),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(10., 0.1, 10.));
}
