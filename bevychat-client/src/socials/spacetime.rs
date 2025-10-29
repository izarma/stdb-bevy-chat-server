use std::collections::VecDeque;

use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_spacetimedb::{StdbConnectedMessage, StdbPlugin};
use spacetimedb_sdk::{Table, Timestamp};

use crate::{
    module_bindings::{
        DbConnection, MessageTableAccess, RemoteTables, UserTableAccess, send_message, set_name,
    },
    socials::{
        ChatState, SpacetimeDB, UserInfo,
        chatui::{LoginMessage, SendMessage},
    },
    utils::{generate_csrf_state, pkce_challenge, pkce_verifier},
};

pub struct SpaceTimePlugin;

impl Plugin for SpaceTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            StdbPlugin::default()
                .with_uri("https://game-server.izaforge.com")
                .with_module_name("bevychat")
                .with_run_fn(DbConnection::run_threaded)
                .add_table(RemoteTables::user)
                .add_table(RemoteTables::message),
        )
        .insert_resource(ChatDataResource::default())
        .add_systems(OnEnter(ChatState::LoggedIn), subscribe_to_messages)
        .add_systems(
            Update,
            (populate_chat_data, handle_send_message_event).run_if(in_state(ChatState::LoggedIn)),
        )
        .add_systems(
            Update,
            (store_token, login_event_handler).run_if(in_state(ChatState::LoggedOut)),
        );
    }
}

#[derive(Resource, Default)]
pub struct ChatDataResource {
    pub msgs: VecDeque<ChatData>,
    last_processed_id: u64,
}

#[derive(Clone, Debug)]
pub struct ChatData {
    pub msg_id: u64,
    pub msg_text: String,
    // pub sender: Identity,
    pub sender_username: String,
    pub timestamp: Timestamp,
}

impl ChatData {
    pub fn new(msg: crate::module_bindings::Message, usr: crate::module_bindings::User) -> Self {
        Self {
            msg_id: msg.id,
            msg_text: msg.text,
            // sender: usr.identity,
            sender_username: usr.name.unwrap(),
            timestamp: msg.sent,
        }
    }
}

fn subscribe_to_messages(stdb: SpacetimeDB) {
    stdb.subscription_builder()
        .on_error(|_, err| error!("Subscription to messages failed for: {}", err))
        .subscribe("SELECT * FROM message");
    stdb.subscription_builder()
        .on_error(|_, err| error!("Subscription to users failed for: {}", err))
        .subscribe("SELECT * FROM user");
}

fn populate_chat_data(mut data: ResMut<ChatDataResource>, stdb: SpacetimeDB) {
    let mut msgs: Vec<_> = stdb
        .db()
        .message()
        .iter()
        .filter(|msg| msg.id > data.last_processed_id)
        .collect();
    if !msgs.is_empty() {
        msgs.sort_by_key(|msg| msg.id);
        for msg in msgs {
            if let Some(usr) = stdb
                .db()
                .user()
                .iter()
                .find(|user| user.identity == msg.sender)
            {
                let msg_data = ChatData::new(msg, usr);
                data.last_processed_id = msg_data.msg_id;
                data.msgs.push_back(msg_data);
                if data.msgs.len() > 50 {
                    data.msgs.pop_front();
                }
            }
        }
    }
}

fn handle_send_message_event(mut events: MessageReader<SendMessage>, stdb: SpacetimeDB) {
    for event in events.read() {
        stdb.reducers().send_message(event.content.clone()).unwrap();
    }
}

fn login_event_handler(
    mut events: MessageReader<LoginMessage>,
    stdb: SpacetimeDB,
    mut state: ResMut<NextState<ChatState>>,
) {
    for event in events.read() {
        match event {
            LoginMessage::Username(usr) => {
                stdb.reducers().set_name(usr.to_string()).unwrap();
                state.set(ChatState::LoggedIn);
            }
            LoginMessage::Discord => {
                let verifier = pkce_verifier();
                let challenge = pkce_challenge(&verifier);
                let state = generate_csrf_state();
                let stdb_client_id = "client_031LPfBM8jHxvge4NX9CNr";
                let redirect_uri = "http://127.0.0.1:42069";
                let auth_url = format!(
                    "https://auth.spacetimedb.com/oidc/auth?client_id={}&redirect_uri={}&scope=openid%20email%20profile&response_type=code&response_mode=query&code_challenge_method=S256&code_challenge={}&state={}",
                    stdb_client_id, redirect_uri, challenge, state
                );
                let expected_state = state.clone();
                IoTaskPool::get()
                    .spawn(async move {
                        info!("Starting server");
                        let server = tiny_http::Server::http("127.0.0.1:42069").unwrap();
                        let request = server.recv().unwrap();
                        let url = request.url();
                        // Parse query parameters from URL
                        let query_string = url.trim_start_matches("/?");
                        let mut code = None;
                        let mut state = None;

                        for param in query_string.split('&') {
                            let parts: Vec<&str> = param.split('=').collect();
                            if parts.len() == 2 {
                                match parts[0] {
                                    "code" => code = Some(parts[1].to_string()),
                                    "state" => state = Some(parts[1].to_string()),
                                    _ => {}
                                }
                            }
                        }
                        if let (Some(received_state), Some(auth_code)) = (state, code) {
                            if received_state == expected_state {
                                info!("Successfully received auth code: {}", auth_code);
                            }
                        }
                    })
                    .detach();
                let _jh = open::that_in_background(auth_url);
            }
        }
    }
}

fn store_token(mut ev_conn: MessageReader<StdbConnectedMessage>, mut user_info: ResMut<UserInfo>) {
    if user_info.space_token.is_none() {
        if let Some(event) = ev_conn.read().next() {
            // Extract the access token from the connection event and store it.
            user_info.space_token = Some(event.access_token.clone());
        }
    }
}
