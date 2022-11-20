use bevy::prelude::*;

mod sprite_plugin;
use sprite_plugin::SpritePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(SpritePlugin)
        .run();
}
