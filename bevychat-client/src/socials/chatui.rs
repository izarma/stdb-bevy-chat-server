use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet, egui};

use crate::{
    module_bindings::set_name,
    socials::{ChatState, SpacetimeDB, UserInfo},
};

pub struct ChatUIPlugin;

impl Plugin for ChatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(
                PreStartup,
                setup_camera_system.before(EguiStartupSet::InitContexts),
            )
            .add_systems(
                EguiPrimaryContextPass,
                show_login_window.run_if(in_state(ChatState::LoggedOut)),
            )
            .add_systems(
                EguiPrimaryContextPass,
                show_main_window.run_if(in_state(ChatState::LoggedIn)),
            );
    }
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn show_login_window(
    mut contexts: EguiContexts,
    mut user_info: ResMut<UserInfo>,
    mut state: ResMut<NextState<ChatState>>,
    stdb: SpacetimeDB,
) -> Result {
    egui::Window::new("Login")
        .vscroll(true)
        .collapsible(false)
        .show(contexts.ctx_mut()?, |ui| {
            ui.label("Set Username");
            ui.text_edit_singleline(&mut user_info.username);
            if ui.add(egui::Button::new("Enter")).clicked() {
                stdb.reducers()
                    .set_name(user_info.username.clone())
                    .unwrap();
                state.set(ChatState::LoggedIn);
            }
        });
    Ok(())
}

fn show_main_window(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Main Window")
        .vscroll(true)
        .collapsible(false)
        .show(contexts.ctx_mut()?, |ui| {
            ui.label("This is the main window!");
        });
    Ok(())
}
