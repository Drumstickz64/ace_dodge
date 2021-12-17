mod player;
mod shared;
mod steerer;

use bevy::prelude::*;
use shared::Materials;

const TITLE: &'static str = "Ace Dodge";
const SCREEN_WIDTH: f32 = 480.0;
const SCREEN_HEIGHT: f32 = 640.0;
const SCREEN_X: i32 = 1305;
const SCREEN_Y: i32 = 0;
const CLEAR_COLOR: Color = Color::rgb(0.47, 0.82, 0.89);
const PLAYER_SPRITE: &'static str = "player_01.png";

fn main() {
    App::build()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            title: TITLE.to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .add_startup_system(position_window.system())
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::parallel())
        .add_plugins(DefaultPlugins)
        .add_plugin(steerer::SteererPlugin)
        .add_plugin(player::PlayerPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let plane_mat_handle = asset_server.load(PLAYER_SPRITE);
    commands.insert_resource(Materials {
        plane_material: materials.add(plane_mat_handle.into()),
    });
}

fn position_window(mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_position(IVec2::new(SCREEN_X, SCREEN_Y));
}
