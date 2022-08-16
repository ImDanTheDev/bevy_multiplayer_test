mod flycam;
mod ui;
use std::time::SystemTime;

use bevy::{
    pbr::DirectionalLightShadowMap,
    prelude::{
        info,
        shape::{Cube, Plane},
        App, Assets, Camera3dBundle, Color, Commands, DirectionalLight, DirectionalLightBundle,
        EventReader, Mesh, PbrBundle, PerspectiveProjection, ResMut, StandardMaterial, Transform,
        Vec3,
    },
    render::camera::Projection,
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use bevy_renet::{renet, RenetClientPlugin};
use game_shared::{Channels, PRIVATE_KEY, PROTOCOL_ID};
use multiplayer_client::shared::MultiplayerConfig;
use ui::client_control_ui;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(DirectionalLightShadowMap {
        size: 8192,
        ..Default::default()
    });
    app.add_plugin(EguiPlugin);

    app.add_plugin(RenetClientPlugin);
    app.insert_resource(MultiplayerConfig::new(60));

    app.add_event::<ConnectEvent>();
    app.add_system(client_control_ui);
    app.add_system(client_connect);
    app.add_system(flycam::flycam_movement);
    app.add_startup_system(setup_scene);
    app.run();
}

fn client_connect(mut commands: Commands, mut connect_events: EventReader<ConnectEvent>) {
    if connect_events.iter().count() == 0 {
        return;
    }

    let server_addr = "127.0.0.1:7777".parse().unwrap();
    let socket = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = renet::RenetConnectionConfig {
        send_channels_config: Channels::channels_config(),
        receive_channels_config: Channels::channels_config(),
        ..Default::default()
    };
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;

    let connect_token = renet::ConnectToken::generate(
        current_time,
        PROTOCOL_ID,
        300,
        client_id,
        15,
        vec![server_addr],
        None,
        PRIVATE_KEY,
    )
    .unwrap();

    let client = renet::RenetClient::new(
        current_time,
        socket,
        client_id,
        connection_config,
        renet::ClientAuthentication::Secure { connect_token },
    )
    .unwrap();

    commands.insert_resource(client);
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane { size: 10.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("3D444D").unwrap(),
            perceptual_roughness: 0.1,
            metallic: 0.5,
            ..Default::default()
        }),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cube::new(1.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("45D767").unwrap(),
            perceptual_roughness: 0.2,
            metallic: 0.7,
            ..Default::default()
        }),
        ..Default::default()
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::hex("ffffff").unwrap(),
            shadows_enabled: true,
            illuminance: 10000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(Camera3dBundle {
        projection: Projection::Perspective(PerspectiveProjection {
            near: 0.1,
            far: 1000.0,
            fov: 45.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

pub struct ConnectEvent;
