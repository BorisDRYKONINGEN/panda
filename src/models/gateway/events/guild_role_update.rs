use crate::models::guild::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildRoleUpdate {
    pub guild_id: String,
    pub role: Role,
}
