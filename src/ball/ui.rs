use bevy::{prelude::*, ui::UiSystem};

use crate::util::cleanup;
use crate::GameState;

use super::{Ball, MAX_BALL_ENERGY};

pub struct BallUiPlugin;

impl Plugin for BallUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BallUi>()
            .register_type::<BallUiBar>()
            .add_system(setup_ball_ui.in_schedule(OnEnter(GameState::Playing)))
            .add_system(
                update_ball_ui
                    .run_if(in_state(GameState::Playing))
                    .in_base_set(CoreSet::PostUpdate)
                    .after(UiSystem::Stack),
            )
            .add_system(cleanup::<BallUi>.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
struct BallUi;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
struct BallUiBar;

fn setup_ball_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(50.0), Val::Px(180.0)),
                flex_direction: FlexDirection::ColumnReverse,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(10.0),
                    right: Val::Px(10.0),
                    ..Default::default()
                },
                padding: UiRect::all(Val::Px(5.)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .insert(BallUi)
        .insert(Name::new("BallUi"))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .insert(BallUiBar)
                .insert(Name::new("BallUiBar"));
        });
}

fn update_ball_ui(ball_query: Query<&Ball>, mut ui_query: Query<&mut Style, With<BallUiBar>>) {
    if let Ok(ball) = ball_query.get_single() {
        let energy = ball.energy;

        for mut style in ui_query.iter_mut() {
            style.size.height = Val::Percent(energy / MAX_BALL_ENERGY * 100.);
        }
    }
}
