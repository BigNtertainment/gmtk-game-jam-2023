mod actions;
mod ball;
mod booster;
mod camera;
mod club;
mod hole;
mod level;
mod light;
mod loading;
mod loading_screen;
mod menu;
mod reset;
mod soundtrack;
mod trampoline;
mod util;
mod win_screen;

use crate::actions::ActionsPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use ball::BallPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use booster::BoosterPlugin;
use camera::CameraPlugin;
use club::ClubPlugin;
use hole::HolePlugin;
use level::LevelPlugin;
use light::LightPlugin;
use loading_screen::LoadingScreenPlugin;
use reset::ResetPlugin;
use soundtrack::SoundtrackPlugin;
use trampoline::TrampolinePlugin;
use win_screen::WinScreenPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // The level is loaded based on the `LevelIndex` resource
    LoadLevel,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // The screen after you complete the game
    Win,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(LoadingScreenPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(LevelPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(LightPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(SoundtrackPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(WinScreenPlugin)
            .add_plugin(ResetPlugin)
            .add_plugin(BallPlugin)
            .add_plugin(HolePlugin)
            .add_plugin(BoosterPlugin)
            .add_plugin(TrampolinePlugin)
            .add_plugin(ClubPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(WorldInspectorPlugin::new())
                .add_plugin(RapierDebugRenderPlugin::default());
        }
    }
}
