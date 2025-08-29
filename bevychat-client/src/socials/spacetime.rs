use bevy::prelude::*;
use bevy_spacetimedb::{ReducerResultEvent, RegisterReducerEvent, StdbPlugin, TableEvents};
use spacetimedb_sdk::ReducerEvent;

use crate::module_bindings::{
    DbConnection, MessageTableAccess, Reducer, RemoteModule, RemoteReducers, RemoteTables,
    UserTableAccess, set_name,
};

pub struct SpaceTimePlugin;

#[derive(Debug, RegisterReducerEvent)]
#[allow(dead_code)]
pub struct SetName {
    event: ReducerEvent<Reducer>,
    username: String,
}

impl Plugin for SpaceTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            StdbPlugin::default()
                .with_uri("https://game-server.izaforge.com")
                .with_module_name("bevychat")
                .with_run_fn(DbConnection::run_threaded)
                .add_table(RemoteTables::user)
                .add_table(RemoteTables::message),
        );
    }
}
