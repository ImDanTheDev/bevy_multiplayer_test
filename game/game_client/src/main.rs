use bevy::{prelude::App, DefaultPlugins};
use multiplayer_client::shared::MultiplayerConfig;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(MultiplayerConfig::new(60));
    app.run();
}
