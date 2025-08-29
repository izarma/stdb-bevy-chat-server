use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet,
    egui::{self, Align2, Layout},
};

use crate::{
    module_bindings::{send_message, set_name},
    socials::{ChatState, SpacetimeDB, UserInfo},
};

pub struct ChatUIPlugin;

impl Plugin for ChatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .insert_resource(UserAction::default())
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

#[derive(Resource, Default, Clone)]
pub struct UserAction {
    message_sent: String,
}

fn setup_camera_system(mut commands: Commands) {
    let main_camera = Camera2d::default();
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
            min_width: (1920.0),
            min_height: (1080.0),
        },
        ..OrthographicProjection::default_2d()
    });
    commands.spawn((main_camera, projection));
}

fn show_login_window(
    mut contexts: EguiContexts,
    mut user_info: ResMut<UserInfo>,
    mut state: ResMut<NextState<ChatState>>,
    stdb: SpacetimeDB,
) -> Result {
    egui::Window::new("Login")
        .collapsible(false)
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .fixed_size([300.0, 200.0])
        .show(contexts.ctx_mut()?, |ui| {
            ui.label("Set Username");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut user_info.username);
                if ui.add(egui::Button::new("Enter")).clicked() {
                    stdb.reducers()
                        .set_name(user_info.username.clone())
                        .unwrap();
                    state.set(ChatState::LoggedIn);
                }
            })
        });
    Ok(())
}

fn show_main_window(
    mut contexts: EguiContexts,
    mut action: ResMut<UserAction>,
    stdb: SpacetimeDB,
) -> Result {
    egui::Window::new("Chat Window")
        .title_bar(false)
        .anchor(Align2::RIGHT_BOTTOM, [-20.0, -20.0])
        .fixed_size([400.0, 300.0])
        .show(contexts.ctx_mut()?, |ui| {
            egui::ScrollArea::vertical()
                .stick_to_right(true) // didnt work
                .scroll_bar_rect(ui.available_rect_before_wrap()) // also didnt work
                .show(ui, |ui| {
                    for i in 0..50 {
                        ui.label(format!("Ghost {}: Hello world", i));
                        ui.label(format!("World {}: Hello Ghost", i));
                    }
                });
            ui.add_space(10.0);
            ui.with_layout(Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    let response = ui.text_edit_singleline(&mut action.message_sent);
                    if ui.add(egui::Button::new("Send")).clicked()
                        || response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        stdb.reducers()
                            .send_message(action.message_sent.clone())
                            .unwrap();
                        action.message_sent.clear();
                    }
                });
            })
        });
    Ok(())
}
