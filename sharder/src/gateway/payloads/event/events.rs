use serde::Deserialize;
use serde_repr::Deserialize_repr;

use chrono::{DateTime, Utc};

use crate::model::user::{User, PresenceUpdate};
use crate::model::guild::{UnavailableGuild, Emoji, Member, Role};
use crate::gateway::ShardInfo;
use crate::model::Snowflake;

#[derive(Deserialize, Debug)]
pub struct Ready {
    #[serde(rename = "v")]
    pub gateway_version: i32,
    pub user: User,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub shard: ShardInfo,
}

#[derive(Deserialize, Debug)]
pub struct ChannelPinsUpdate {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct GuildBanAdd {
    pub guild_id: Snowflake,
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct GuildBanRemove {
    pub guild_id: Snowflake,
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct GuildEmojisUpdate {
    pub guild_id: Snowflake,
    pub emojis: Vec<Emoji>,
}

#[derive(Deserialize, Debug)]
pub struct GuildIntegrationsUpdate {
    pub guild_id: Snowflake,
}

#[derive(Deserialize, Debug)]
pub struct GuildMemberAdd {
    pub guild_id: Snowflake,
    #[serde(flatten)]
    pub member: Member,
}

#[derive(Deserialize, Debug)]
pub struct GuildMemberRemove {
    pub guild_id: Snowflake,
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct GuildMemberUpdate {
    pub guild_id: Snowflake,
    pub roles: Vec<Snowflake>,
    pub user: User,
    pub nick: Option<String>,
    pub joined_at: DateTime<Utc>,
    pub premium_since: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct GuildMembersChunk {
    pub guild_id: Snowflake,
    pub members: Vec<Member>,
    pub chunk_index: u32,
    pub chunk_count: u32,
    pub not_found: Option<Vec<Snowflake>>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub nonce: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GuildRoleCreate {
    pub guild_id: Snowflake,
    pub role: Role,
}

#[derive(Deserialize, Debug)]
pub struct GuildRoleUpdate {
    pub guild_id: Snowflake,
    pub role: Role,
}

#[derive(Deserialize, Debug)]
pub struct GuildRoleDelete {
    pub guild_id: Snowflake,
    pub role_id: Snowflake,
}

#[derive(Deserialize, Debug)]
pub struct InviteCreate {
    pub channel_id: Snowflake,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub guild_id: Option<Snowflake>,
    pub inviter: Option<User>,
    pub max_age: usize,
    pub max_uses: u32,
    pub target_user: Option<User>,
    pub target_user_type: Option<TargetUserType>,
    pub temporary: bool,
    pub uses: u8, // we can use u8 because it will always initially be 0
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum TargetUserType {
    Stream = 1,
}

#[derive(Deserialize, Debug)]
pub struct InviteDelete {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub code: String,
}

#[derive(Deserialize, Debug)]
pub struct MessageDelete {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Deserialize, Debug)]
pub struct MessageDeleteBulk {
    pub ids: Vec<Snowflake>,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Deserialize, Debug)]
pub struct MessageReactionAdd {
    pub user_id: Snowflake,
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub member: Option<Member>,
    pub emoji: Emoji,
}

#[derive(Deserialize, Debug)]
pub struct MessageReactionRemove {
    pub user_id: Snowflake,
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub emoji: Emoji,
}

#[derive(Deserialize, Debug)]
pub struct MessageReactionRemoveAll {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Deserialize, Debug)]
pub struct MessageReactionRemoveEmoji {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub message_id: Snowflake,
    pub emoji: Emoji,
}

#[derive(Deserialize, Debug)]
pub struct TypingStart {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub timestamp: u64,
    pub member: Option<Member>,
}

#[derive(Deserialize, Debug)]
pub struct VoiceServerUpdate {
    pub token: String,
    pub guild_id: Snowflake,
    pub endpoint: String,
}

#[derive(Deserialize, Debug)]
pub struct WebhooksUpdate {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
}
