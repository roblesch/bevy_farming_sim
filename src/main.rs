mod game;
use bevy::prelude::*;
use crate::game::FarmingSimPlugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FarmingSimPlugins,
        ))
        .run();
}
