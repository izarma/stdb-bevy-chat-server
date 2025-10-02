use spacetimedb::{table, Identity, SpacetimeType, Timestamp};

mod chat;
mod connect;

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    identity: Identity,
    name: Option<String>,
    online: bool,
    discord_link: Option<DiscordUserLink>,
}

#[derive(SpacetimeType)]
pub struct DiscordUserLink {
    pub id: String,
    pub roles: Vec<String>,
    pub token: String,
}

#[table(name = message, public)]
pub struct Message {
    #[primary_key]
    #[auto_inc]
    id: u64,
    sender: Identity,
    sent: Timestamp,
    text: String,
}
