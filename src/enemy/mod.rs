mod common;
mod red_enemy;
mod yellow_enemy;

use bevy::{app::PluginGroupBuilder, prelude::*};
use common::CommonEnemyPlugin;
use red_enemy::RedEnemyPlugin;
use yellow_enemy::YellowEnemyPlugin;

pub struct EnemyPlugins;

impl PluginGroup for EnemyPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(CommonEnemyPlugin)
            .add(RedEnemyPlugin)
            .add(YellowEnemyPlugin);
    }
}
