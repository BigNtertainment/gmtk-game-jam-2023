use bevy::prelude::*;

use crate::{
    loading::{FontAssets, TextureAssets},
    menu::{ButtonColors, button_colors},
    GameState, util::cleanup,
};

pub struct WinScreenPlugin;

impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_win_screen.in_schedule(OnEnter(GameState::Win)))
            .add_systems((go_to_menu, button_colors).in_set(OnUpdate(GameState::Win)))
            .add_systems(
                (cleanup::<Camera2d>, cleanup::<WinScreen>).in_schedule(OnExit(GameState::Win)),
            );
    }
}

#[derive(Component, Clone, Copy, Debug)]
struct WinScreen;

#[derive(Component, Clone, Copy, Debug)]
struct GoToMenuButton;

fn spawn_win_screen(
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
        .insert(WinScreen)
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

                    parent.spawn(
                        TextBundle::from_section(
                            "Congratulations! You completed the game.",
                            TextStyle {
                                font: font_assets.poppins.clone(),
                                font_size: 32.,
                                color: Color::BLACK,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(64.)),
                            ..default()
                        }),
                    );

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
                        .insert(GoToMenuButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Go to menu",
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

fn go_to_menu(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<GoToMenuButton>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            state.set(GameState::Menu);
        }
    }
}
