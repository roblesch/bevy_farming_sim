mod grid;
mod data;
mod gameplay;

use bevy::{app::PluginGroupBuilder, prelude::*};
use crate::game::grid::GridPlugin;
use crate::game::data::DataPlugin;
use crate::game::gameplay::GameplayPlugin;

pub struct FarmingSimPlugins;

impl PluginGroup for FarmingSimPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GridPlugin)
            .add(DataPlugin)
            .add(GameplayPlugin)
    }
}