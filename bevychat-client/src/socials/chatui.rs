use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet,
    egui::{self, Align2, Color32, FontId, Layout, RichText},
};
use spacetimedb_sdk::Timestamp;

use crate::{
    module_bindings::set_name,
    socials::{ChatState, SpacetimeDB, UserInfo, spacetime::ChatDataResource},
};

pub struct ChatUIPlugin;

impl Plugin for ChatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .insert_resource(UserAction::default())
            .add_event::<SendMessageEvent>()
            .add_event::<LoginEvent>()
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
    currently_typing: String,
}

#[derive(Event)]
pub struct SendMessageEvent {
    pub content: String,
}

#[derive(Event)]
pub enum LoginEvent {
    Username(String),
    Discord,
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
    mut login: EventWriter<LoginEvent>,
) -> Result {
    egui::Window::new("Login")
        .collapsible(false)
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .fixed_size([300.0, 200.0])
        .show(contexts.ctx_mut()?, |ui| {
            ui.label("Set Username");
            ui.horizontal(|ui| {
                let response = ui.text_edit_singleline(&mut user_info.username);
                if ui.add(egui::Button::new("Enter")).clicked()
                    || response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    login.write(LoginEvent::Username(user_info.username.clone()));
                    
                }
            });
            if ui.add(egui::Button::new("Login with Discord")).clicked() {
                login.write(LoginEvent::Discord);
            }
        });
    Ok(())
}

fn show_main_window(
    mut contexts: EguiContexts,
    mut action: ResMut<UserAction>,
    mut send_msg: EventWriter<SendMessageEvent>,
    chat_data: Res<ChatDataResource>,
) -> Result {
    egui::Window::new("Chat Window")
        .title_bar(false)
        .anchor(Align2::RIGHT_BOTTOM, [-20.0, -20.0])
        .fixed_size([700.0, 300.0])
        .show(contexts.ctx_mut()?, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for msg in &chat_data.msgs {
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::left_to_right(egui::Align::LEFT), |ui| {
                            ui.label(
                                RichText::new(format!(
                                    "{} : {}",
                                    msg.sender_username, msg.msg_text
                                ))
                                .font(FontId::proportional(14.0))
                                .color(Color32::WHITE),
                            );
                        });
                        ui.with_layout(Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            ui.label(
                                RichText::new(format!("{}", get_formatted_time(msg.timestamp)))
                                    .font(FontId::proportional(12.0))
                                    .color(Color32::GRAY),
                            );
                        });
                    });
                }
            });
            ui.add_space(10.0);
            ui.with_layout(Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    let response = ui.text_edit_singleline(&mut action.currently_typing);
                    if ui.add(egui::Button::new("Send")).clicked()
                        || response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        send_msg.write(SendMessageEvent {
                            content: action.currently_typing.clone(),
                        });
                        action.currently_typing.clear();
                    }
                });
            })
        });
    Ok(())
}

fn get_formatted_time(time: Timestamp) -> String {
    time.to_rfc3339().unwrap_or_default()[11..19].to_string()
}
