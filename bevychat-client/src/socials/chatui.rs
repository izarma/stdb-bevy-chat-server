use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet, egui};

use crate::consts::{BUTTON_BORDER, NORMAL_BUTTON, TEXT_COLOR};

pub struct ChatUIPlugin;

impl Plugin for ChatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(
                PreStartup,
                setup_camera_system.before(EguiStartupSet::InitContexts),
            )
            .add_systems(EguiPrimaryContextPass, ui_example_system);
    }
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn ui_example_system(mut contexts: EguiContexts) -> Result {
    let mut user_input = String::new();
    egui::Window::new("Login")
        .vscroll(true)
        .collapsible(false)
        .show(contexts.ctx_mut()?, |ui| {
            ui.label("Set Username");
            ui.text_edit_singleline(&mut user_input);
        });
    Ok(())
}
