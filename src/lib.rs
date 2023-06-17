mod actions;
mod audio;
mod levels;
mod loading;
mod menu;
mod player;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::levels::LevelsPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // Here the menu is drawn and waiting for player interaction
    StartMenu,
    // During this State the actual game logic is executed
    Playing,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum Level {
    #[default]
    One,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(InternalAudioPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_state::<Level>()
            .add_plugin(LevelsPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
