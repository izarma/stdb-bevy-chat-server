use bevy::prelude::*;
use bevy_spacetimedb::StdbConnection;

use crate::{
    module_bindings::DbConnection,
    socials::{chatui::ChatUIPlugin, discord::DiscordPlugin, spacetime::SpaceTimePlugin},
};

pub mod chatui;
pub mod spacetime;
pub mod discord;

pub struct SocialsPlugin;

pub type SpacetimeDB<'a> = Res<'a, StdbConnection<DbConnection>>;

#[derive(Resource, Default, Clone)]
pub struct UserInfo {
    username: String,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum ChatState {
    #[default]
    LoggedOut,
    LoggedIn,
}

impl Plugin for SocialsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UserInfo::default())
            .init_state::<ChatState>()
            .add_plugins((SpaceTimePlugin, ChatUIPlugin, DiscordPlugin));
    }
}
