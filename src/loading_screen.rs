use bevy::prelude::*;

use crate::GameState;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_loading_screen.in_schedule(OnEnter(GameState::Loading)))
            .add_system(animate_dots.in_set(OnUpdate(GameState::Loading)))
            .add_system(cleanup_loading_screen.in_schedule(OnExit(GameState::Loading)));
    }
}

#[derive(Component)]
pub struct LoadingScreenUi;

#[derive(Component)]
pub struct LoadingTextTimer(Timer);

fn setup_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.)),
                padding: UiRect::all(Val::Px(32.)),
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(LoadingScreenUi)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_sections([
                    TextSection::new(
                        "Loading ",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        ".",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ),
                ]))
                .insert(LoadingTextTimer(Timer::from_seconds(
                    0.5,
                    TimerMode::Repeating,
                )));
        });
}

fn animate_dots(mut query: Query<(&mut Text, &mut LoadingTextTimer)>, time: Res<Time>) {
    for (mut text, mut loading_text_timer) in &mut query {
        if loading_text_timer.0.tick(time.delta()).just_finished() {
            match text.sections[1].value.as_str() {
                "." => text.sections[1].value = "..".to_string(),
                ".." => text.sections[1].value = "...".to_string(),
                "..." => text.sections[1].value = ".".to_string(),
                _ => {}
            }
        }
    }
}

fn cleanup_loading_screen(
    mut commands: Commands,
    camera: Query<Entity, With<Camera>>,
    ui: Query<Entity, With<LoadingScreenUi>>,
) {
    commands.entity(ui.single()).despawn_recursive();
    commands.entity(camera.single()).despawn();
}
