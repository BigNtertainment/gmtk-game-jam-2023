use crate::level::LevelIndex;
use crate::loading::{FontAssets, TextureAssets};
use crate::util::cleanup;
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_systems((play_button, button_colors).in_set(OnUpdate(GameState::Menu)))
            .add_systems(
                (cleanup::<Menu>, cleanup::<Camera2d>).in_schedule(OnExit(GameState::Menu)),
            );
    }
}

#[derive(Component, Clone, Copy, Debug)]
struct Menu;

#[derive(Component, Clone, Copy, Debug)]
struct PlayButton;

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(512.), Val::Auto),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Px(64.)),
                            margin: UiRect::bottom(Val::Px(128.)),
                            ..default()
                        },
                        image: UiImage::new(texture_assets.logo.clone()),
                        ..default()
                    });

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(32.), Val::Px(48.)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: button_colors.normal.into(),
                            ..default()
                        })
                        .insert(PlayButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play",
                                TextStyle {
                                    font: font_assets.poppins.clone(),
                                    font_size: 32.,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        });
}

fn button_colors(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {}
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn play_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut state: ResMut<NextState<GameState>>,
    mut level_index: ResMut<LevelIndex>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            level_index.0 = 0;
            state.set(GameState::LoadLevel);
        }
    }
}
