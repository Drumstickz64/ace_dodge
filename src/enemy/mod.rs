mod common;
mod red_enemy;
mod yellow_enemy;

use bevy::prelude::*;
use common::CommonEnemyPlugin;
use red_enemy::RedEnemyPlugin;
use yellow_enemy::YellowEnemyPlugin;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(CommonEnemyPlugin);
        app.add_plugin(RedEnemyPlugin);
        app.add_plugin(YellowEnemyPlugin);
    }
}
