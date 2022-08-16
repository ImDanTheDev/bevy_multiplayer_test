use std::time::SystemTime;

use bevy::{
    log::LogPlugin,
    prelude::{info, App, Commands, EventReader},
    MinimalPlugins,
};
use bevy_renet::{
    renet::{self},
    RenetServerPlugin,
};
use game_shared::{Channels, PRIVATE_KEY, PROTOCOL_ID};
use multiplayer_server::shared::MultiplayerConfig;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugin(LogPlugin);

    app.add_plugin(RenetServerPlugin);
    app.insert_resource(MultiplayerConfig::new(60));

    app.add_system(server_events);
    app.add_startup_system(start_server);
    app.run();
}

fn server_events(mut events: EventReader<renet::ServerEvent>) {
    for event in events.iter() {
        match event {
            renet::ServerEvent::ClientConnected(client_id, _client_data) => {
                info!("Client Connected: {}", client_id);
            }
            renet::ServerEvent::ClientDisconnected(client_id) => {
                info!("Client Disconnected: {}", client_id);
            }
        }
    }
}

fn start_server(mut commands: Commands) {
    let server_addr = "127.0.0.1:7777".parse().unwrap();
    let socket = std::net::UdpSocket::bind(server_addr).unwrap();
    let connection_config = renet::RenetConnectionConfig {
        send_channels_config: Channels::channels_config(),
        receive_channels_config: Channels::channels_config(),
        ..Default::default()
    };
    let server_config = renet::ServerConfig::new(
        4,
        PROTOCOL_ID,
        server_addr,
        renet::ServerAuthentication::Secure {
            private_key: *PRIVATE_KEY,
        },
    );
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let server =
        renet::RenetServer::new(current_time, server_config, connection_config, socket).unwrap();
    commands.insert_resource(server);
}
