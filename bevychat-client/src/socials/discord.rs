use bevy::prelude::*;
use bevy_http_client::HttpClientPlugin;

pub struct DiscordPlugin;

impl Plugin for DiscordPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HttpClientPlugin);
    }
}