use bevy::prelude::*;
use bevy_spacetimedb::{RegisterReducerEvent, StdbPlugin, ReducerResultEvent};
use spacetimedb_sdk::ReducerEvent;

use crate::module_bindings::{set_name, DbConnection, Reducer, RemoteTables, UserTableAccess, RemoteModule, RemoteReducers};

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
                .add_reducer::<SetName>()
        );
    }
}
