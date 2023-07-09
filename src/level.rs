use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, Friction};
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

use crate::{ball::BallBundle, hole::Hole, loading::ModelAssets, util::cleanup, GameState, booster::Booster};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelIndex>()
            .add_plugin(HookPlugin)
            .add_system(load_level.in_schedule(OnEnter(GameState::LoadLevel)))
            .add_system(update_colliders.in_set(OnUpdate(GameState::Playing)))
            .add_system(cleanup::<LevelTag>.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Resource, Clone, Copy, Debug, Deref, DerefMut, Default)]
pub struct LevelIndex(pub usize);

#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct LevelTag;

#[derive(Component, Clone, Debug)]
struct UpdateCollider {
    pub mesh: Handle<Mesh>,
    pub parent: Entity,
}

fn load_level(
    mut commands: Commands,
    models: Res<ModelAssets>,
    level_index: Res<LevelIndex>,
    mut state: ResMut<NextState<GameState>>,
) {
    commands
        .spawn(HookedSceneBundle {
            scene: SceneBundle {
                scene: models.levels[level_index.0].clone(),
                ..default()
            },
            hook: SceneHook::new(|entity, commands| {
                match entity.get::<Name>().map(|name| name.as_str()) {
                    Some("ball") => {
                        commands.insert(BallBundle::default());
                    }
                    Some("hole") => {
                        commands.insert(Hole);
                    }
                    Some("speed") => {
                        commands.insert(Booster::default());
                    }
                    _ => {
                        let mesh = entity.get::<Handle<Mesh>>();
                        let parent = entity.get::<Parent>();

                        if let Some(mesh) = mesh {
                            commands
                                .insert(UpdateCollider {
                                    mesh: mesh.clone(),
                                    parent: parent.unwrap().get(),
                                })
                                .insert(Friction::new(1.));
                        }
                    }
                }
            }),
        })
        .insert(LevelTag)
        .insert(Name::new("level"));

    state.set(GameState::Playing);
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
