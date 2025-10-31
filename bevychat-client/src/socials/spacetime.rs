use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_http_client::HttpClientPlugin;
use bevy_spacetimedb::{StdbConnectedMessage, StdbPlugin};
use spacetimedb_sdk::{Table, Timestamp};

use crate::{
    module_bindings::{
        DbConnection, MessageTableAccess, RemoteTables, UserTableAccess, send_message,
    },
    socials::{
        ChatState, SpacetimeDB, UserInfo,
        chatui::SendMessage,
        login::{LoginData, handle_error, handle_response, login_event_handler},
    },
};

pub struct SpaceTimePlugin;

impl Plugin for SpaceTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HttpClientPlugin);
        app.add_plugins(
            StdbPlugin::default()
                .with_uri("https://game-server.izaforge.com")
                .with_module_name("bevychat")
                .with_run_fn(DbConnection::run_threaded)
                // .with_token(LoginData.access_token.clone())
                .add_table(RemoteTables::user)
                .add_table(RemoteTables::message),
        )
        .insert_resource(ChatDataResource::default())
        .insert_resource(LoginData::default())
        .add_systems(OnEnter(ChatState::LoggedIn), subscribe_to_messages)
        .add_systems(
            Update,
            (populate_chat_data, handle_send_message_event).run_if(in_state(ChatState::LoggedIn)),
        )
        .add_systems(
            Update,
            (
                store_token,
                login_event_handler,
                handle_response,
                handle_error,
            )
                .run_if(in_state(ChatState::LoggedOut)),
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

fn store_token(mut ev_conn: MessageReader<StdbConnectedMessage>, mut user_info: ResMut<UserInfo>) {
    if user_info.space_token.is_none() {
        if let Some(event) = ev_conn.read().next() {
            // Extract the access token from the connection event and store it.
            user_info.space_token = Some(event.access_token.clone());
        }
    }
}
