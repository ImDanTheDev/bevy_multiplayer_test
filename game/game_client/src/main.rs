mod flycam;
use bevy::{
    prelude::{
        shape::Cube, App, Assets, Camera3dBundle, Color, Commands, DirectionalLight,
        DirectionalLightBundle, Mesh, PbrBundle, PerspectiveProjection, ResMut, StandardMaterial,
        Transform, Vec3,
    },
    render::camera::Projection,
    DefaultPlugins,
};
use multiplayer_client::shared::MultiplayerConfig;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(MultiplayerConfig::new(60));
    app.add_system(flycam::flycam_movement);
    app.add_startup_system(setup_scene);
    app.run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cube::new(1.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("1e2227").unwrap(),
            perceptual_roughness: 0.35,
            metallic: 0.5,
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
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(Camera3dBundle {
        projection: Projection::Perspective(PerspectiveProjection {
            near: 0.1,
            far: 1000.0,
            fov: 45.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
