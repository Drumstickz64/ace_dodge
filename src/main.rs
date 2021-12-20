mod collision;
mod enemy;
mod math;
mod player;
mod shared;
mod steerer;

use bevy::app::AppExit;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use enemy::EnemyPlugins;
use player::PlayerPlugin;
use shared::{Fonts, Materials, Score};
use steerer::SteererPlugin;

const TITLE: &'static str = "Ace Dodge";
const SCREEN_WIDTH: f32 = 480.0;
const SCREEN_HEIGHT: f32 = 640.0;
const SCREEN_X: i32 = 1305;
const SCREEN_Y: i32 = 0;
const INCREMENT_SCORE_STEP: f64 = 1.0;

const CLEAR_COLOR: Color = Color::rgb(0.47, 0.82, 0.89);

const PLAYER_SPRITE: &'static str = "player_01.png";
const RED_ENEMY_SPRITE: &'static str = "red_enemy_01.png";
const YELLOW_ENEMY_SPRITE: &'static str = "yellow_enemy_01.png";
const FONT: &str = "Montserrat-Bold.ttf";

/*
GOALS:
    [X] add score
    [X] add gui
    [] add start and gameover states
    [] Maybe: add increasing difficulty
    [] Maybe: add audio
*/

fn main() {
    App::build()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            title: TITLE.to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .insert_resource(Score(0))
        .add_startup_stage("setup", SystemStage::parallel())
        .add_startup_system_to_stage("setup", position_window.system())
        .add_startup_system_to_stage("setup", setup.system())
        .add_startup_stage_after("setup", "prelude", SystemStage::parallel())
        .add_startup_system_to_stage("prelude", spawn_ui.system())
        .add_system(handle_esc.system())
        .add_system(
            increment_score
                .system()
                .with_run_criteria(FixedTimestep::step(INCREMENT_SCORE_STEP)),
        )
        .add_system(update_score_label.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(SteererPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugins(EnemyPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let player_mat_handle = asset_server.load(PLAYER_SPRITE);
    let red_enemy_mat_handle = asset_server.load(RED_ENEMY_SPRITE);
    let yellow_enemy_mat_handle = asset_server.load(YELLOW_ENEMY_SPRITE);
    commands.insert_resource(Materials {
        plane_material: materials.add(player_mat_handle.into()),
        red_enemy_material: materials.add(red_enemy_mat_handle.into()),
        yellow_enemy_material: materials.add(yellow_enemy_mat_handle.into()),
    });
    let font_handle = asset_server.load(FONT);
    commands.insert_resource(Fonts {
        main_font: font_handle,
    })
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

struct ScoreLabel;

fn spawn_ui(mut commands: Commands, fonts: Res<Fonts>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "0",
                TextStyle {
                    font: fonts.main_font.clone(),
                    font_size: 48.0,
                    ..Default::default()
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            style: Style {
                display: Display::Flex,
                margin: Rect {
                    left: Val::Auto,
                    top: Val::Percent(5.0),
                    right: Val::Auto,
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreLabel);
}

// ideally would be using an event
fn update_score_label(mut label: Query<&mut Text, With<ScoreLabel>>, score: Res<Score>) {
    for mut text in label.iter_mut() {
        text.sections[0].value = format!("{}", score.0);
    }
}

fn increment_score(mut score: ResMut<Score>) {
    score.increase_by(1);
}
