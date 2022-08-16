use bevy::{
    input::mouse::MouseMotion,
    prelude::{
        Camera, EventReader, Input, KeyCode, Local, MouseButton, Quat, Query, Res, Transform, Vec3,
        With,
    },
    time::Time,
};

pub fn flycam_movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    mut px: Local<f32>,
    mut py: Local<f32>,
    mut input_disabled: Local<bool>,
) {
    if input.just_pressed(KeyCode::F1) {
        *input_disabled = !*input_disabled;
    }
    if *input_disabled {
        return;
    }
    if buttons.pressed(MouseButton::Right) {
        for ev in motion_evr.iter() {
            *py -= ev.delta.x * 0.1;
            *px -= ev.delta.y * 0.1;
        }
    }

    let x_input = match (input.pressed(KeyCode::A), input.pressed(KeyCode::D)) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        _ => 0.0,
    };

    let z_input = match (input.pressed(KeyCode::W), input.pressed(KeyCode::S)) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        _ => 0.0,
    };

    let y_input = match (
        input.pressed(KeyCode::Space),
        input.pressed(KeyCode::LControl),
    ) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    };

    let look_rotation = Quat::from_axis_angle(Vec3::Y, py.to_radians())
        * Quat::from_axis_angle(Vec3::X, px.to_radians());
    let input_direction = Vec3::new(x_input, y_input, z_input).normalize_or_zero();
    let movement_direction = look_rotation * input_direction;

    for mut transform in query.iter_mut() {
        transform.rotation = look_rotation;

        transform.translation += movement_direction * 4.0 * time.delta_seconds();
    }
}
