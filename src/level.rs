use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape};
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

use crate::{ball::BallBundle, loading::ModelAssets, GameState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HookPlugin)
            .add_system(load_level.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update_colliders.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component, Clone, Debug)]
struct UpdateCollider {
    pub mesh: Handle<Mesh>,
    pub parent: Entity,
}

fn load_level(mut commands: Commands, models: Res<ModelAssets>) {
    commands
        .spawn(HookedSceneBundle {
            scene: SceneBundle {
                scene: models.level.clone(),
                ..default()
            },
            hook: SceneHook::new(|entity, commands| {
                match entity.get::<Name>().map(|name| name.as_str()) {
                    Some("ball") => {
                        commands.insert(BallBundle::default());
                    }
                    _ => {
                        let mesh = entity.get::<Handle<Mesh>>();
                        let parent = entity.get::<Parent>();

                        if let Some(mesh) = mesh {
                            commands.insert(UpdateCollider {
                                mesh: mesh.clone(),
                                parent: parent.unwrap().get(),
                            });
                        }
                    }
                }
            }),
        })
        .insert(Name::new("level"));
}

fn update_colliders(
    mut commands: Commands,
    query: Query<(Entity, &UpdateCollider)>,
    colliders: Query<(), With<Collider>>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    for (entity, update_collider) in query.iter() {
        // Don't add a collider if the parent already has a collider
        if colliders.get(update_collider.parent).is_ok() {
            continue;
        }

        let mesh = mesh_assets.get(&update_collider.mesh).unwrap();

        commands.entity(entity).insert(
            Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh)
                .expect("couldn't generate a collider"),
        );

        commands.entity(entity).remove::<UpdateCollider>();
    }
}
