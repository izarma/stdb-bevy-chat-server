use bevy::prelude::*;
use bevy_spacetimedb::{StdbConnection, StdbPlugin, TableEvents};

use crate::module_bindings::{DbConnection, MessageTableAccess, RemoteTables, UserTableAccess};

pub struct SocialsPlugin;

pub type SpacetimeDB<'a> = Res<'a, StdbConnection<DbConnection>>;

impl Plugin for SocialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            StdbPlugin::default()
            .with_uri("https://game-server.izaforge.com")
            .with_module_name("bevychat")
            .with_run_fn(DbConnection::run_threaded)
            .add_table(RemoteTables::user)
        );
    }
}