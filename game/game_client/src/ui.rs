use bevy::prelude::EventWriter;
use bevy::prelude::ResMut;
use bevy_egui::egui;
use bevy_egui::EguiContext;

use crate::ConnectEvent;

pub fn client_control_ui(
    mut ctx: ResMut<EguiContext>,
    mut connect_events: EventWriter<ConnectEvent>,
) {
    egui::Window::new("Client Control").show(ctx.ctx_mut(), |ui| {
        if ui.button("Connect").clicked() {
            connect_events.send(ConnectEvent);
        }
    });
}
