use bevy::prelude::default;
use renet::{ChannelConfig, ReliableChannelConfig, UnreliableChannelConfig};

pub const PRIVATE_KEY: &[u8; 32] = b"01234567890123456789012345678901";
pub const PROTOCOL_ID: u64 = 1;

pub enum Channels {
    Input,
    State,
    Snapshot,
    Request,
}

impl Channels {
    pub fn id(&self) -> u8 {
        match self {
            Self::Input => 0,
            Self::State => 1,
            Self::Snapshot => 2,
            Self::Request => 3,
        }
    }

    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig::Unreliable(UnreliableChannelConfig {
                channel_id: Self::Input.id(),
                ..default()
            }),
            ChannelConfig::Unreliable(UnreliableChannelConfig {
                channel_id: Self::State.id(),
                ..default()
            }),
            ChannelConfig::Reliable(ReliableChannelConfig {
                channel_id: Self::Snapshot.id(),
                ..default()
            }),
            ChannelConfig::Reliable(ReliableChannelConfig {
                channel_id: Self::Request.id(),
                ..default()
            }),
        ]
    }
}
