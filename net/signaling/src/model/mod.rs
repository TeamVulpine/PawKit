use core::fmt;
use std::str::FromStr;

use pawkit_crockford::{FromCrockford, IntoCrockford};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

pub mod c2s;
pub mod s2c;

#[derive(Debug, Serialize, Deserialize)]
pub enum SignalingError {
    InvalidExpectedMessage,
    UnknownClientId,
    UnknownHostId,
    InternalError,
}

#[derive(Debug, Clone)]
pub struct HostId {
    pub server_url: String,
    pub lobby_id: u32,
    pub shard_id: u8,
}

fn shorten_server_url(server_url: &str) -> Option<&str> {
    const SUFFIX: &str = ".signaling.teamvulpine.com";

    if let Some(rest) = server_url.strip_prefix("wss://") {
        if let Some(region) = rest.strip_suffix(SUFFIX) {
            return Some(region);
        }
    } else if let Some(rest) = server_url.strip_prefix("ws://") {
        if let Some(region) = rest.strip_suffix(SUFFIX) {
            return Some(region);
        }
    }

    None
}

impl fmt::Display for HostId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shard_str = format!("{:02X}", self.shard_id);

        let game_str = self.lobby_id.into_crockford(7);

        let server_url = shorten_server_url(&self.server_url).unwrap_or(&self.server_url);

        write!(f, "{}:{}@{}", shard_str, game_str, server_url)
    }
}

impl FromStr for HostId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('@').collect();
        if parts.len() != 2 {
            return Err("Missing '@'".into());
        }

        let (left, server_input) = (parts[0], parts[1]);

        let mut left_parts = left.split(':');
        let shard_hex = left_parts.next().ok_or("Missing shard")?;
        let game_crock = left_parts.next().ok_or("Missing game id")?;

        if left_parts.next().is_some() {
            return Err("Too many ':' segments".into());
        }

        let shard_id = u8::from_str_radix(shard_hex, 16).map_err(|_| "Invalid shard hex")?;
        let game_id = u32::from_crockford(game_crock).ok_or("Invalid Crockford game id")?;

        let server_url = if server_input.contains('.') || server_input.contains(':') {
            if server_input.starts_with("ws://") || server_input.starts_with("wss://") {
                server_input.to_string()
            } else {
                format!("wss://{}", server_input)
            }
        } else {
            format!("wss://{}.signaling.teamvulpine.com", server_input)
        };

        Ok(HostId {
            server_url,
            lobby_id: game_id,
            shard_id,
        })
    }
}

impl Serialize for HostId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for HostId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostIdVisitor;

        impl<'de> Visitor<'de> for HostIdVisitor {
            type Value = HostId;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string in format 'SHARD:GID@URL'")
            }

            fn visit_str<E>(self, value: &str) -> Result<HostId, E>
            where
                E: de::Error,
            {
                value.parse().map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(HostIdVisitor)
    }
}
