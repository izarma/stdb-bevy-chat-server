use bevy::prelude::*;
use bevy_spacetimedb::StdbPlugin;

use crate::module_bindings::{DbConnection, MessageTableAccess, RemoteTables, UserTableAccess};

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
        );
    }
}
