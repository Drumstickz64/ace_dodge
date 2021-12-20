mod collision;
mod enemy;
mod math;
mod player;
mod shared;
mod steerer;

use bevy::app::AppExit;
use bevy::prelude::*;
use shared::Materials;

const TITLE: &'static str = "Ace Dodge";
const SCREEN_WIDTH: f32 = 480.0;
const SCREEN_HEIGHT: f32 = 640.0;
const SCREEN_X: i32 = 1305;
const SCREEN_Y: i32 = 0;

const CLEAR_COLOR: Color = Color::rgb(0.47, 0.82, 0.89);

const PLAYER_SPRITE: &'static str = "player_01.png";
const RED_ENEMY_SPRITE: &'static str = "red_enemy_01.png";
const YELLOW_ENEMY_SPRITE: &'static str = "yellow_enemy_01.png";

fn main() {
    App::build()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            title: TITLE.to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .add_startup_stage("setup", SystemStage::parallel())
        .add_startup_system_to_stage("setup", position_window.system())
        .add_startup_system_to_stage("setup", setup.system())
        .add_startup_stage_after("setup", "prelude", SystemStage::parallel())
        .add_system(handle_esc.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(steerer::SteererPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let plane_mat_handle = asset_server.load(PLAYER_SPRITE);
    let red_enemy_mat_handle = asset_server.load(RED_ENEMY_SPRITE);
    let yellow_enemy_mat_handle = asset_server.load(YELLOW_ENEMY_SPRITE);
    commands.insert_resource(Materials {
        plane_material: materials.add(plane_mat_handle.into()),
        red_enemy_material: materials.add(red_enemy_mat_handle.into()),
        yellow_enemy_material: materials.add(yellow_enemy_mat_handle.into()),
    });
}

fn position_window(mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_position(IVec2::new(SCREEN_X, SCREEN_Y));
}

fn handle_esc(keyboard_input: Res<Input<KeyCode>>, mut exit_writer: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit_writer.send(AppExit);
    }
}
