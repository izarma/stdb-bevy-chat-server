use bevy::prelude::*;

use crate::socials::{chatui::ChatUIPlugin, spacetime::SpaceTimePlugin};

pub mod chatui;
pub mod spacetime;

pub struct SocialsPlugin;

impl Plugin for SocialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpaceTimePlugin, ChatUIPlugin));
    }
}
