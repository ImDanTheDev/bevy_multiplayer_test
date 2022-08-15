use bevy::{prelude::App, MinimalPlugins};
use multiplayer_server::shared::MultiplayerConfig;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(MultiplayerConfig::new(60));
    app.run();
}
