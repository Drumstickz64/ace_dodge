mod collision;
mod enemy;
mod math;
mod player;
mod shared;
mod steerer;
mod ui;

use bevy::app::AppExit;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};
use enemy::EnemyPlugins;
use player::PlayerPlugin;
use shared::{BGMusic, Fonts, Materials, Score, SoundEffects};
use steerer::SteererPlugin;
use ui::UiPlugin;

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
const BG_MUSIC: &str = "flight_of_the_bumblebee.mp3";
const PLANE_SFX: &str = "airplane_sound.mp3";

/*
GOALS:
    [X] add score
    [X] add gui
    // as of bevy 0.5, this is extremely annoying to implement because state is implemented using
    // run criteria and there is no easy way to have multiple run criteria on the same SystemSet
    [] add start and gameover states
    [X] Maybe: add increasing difficulty
    [X] Maybe: add audio
    [] Maybe: add warning before enemy spawns
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
        .add_startup_system_to_stage("setup", setup_window.system())
        .add_startup_system_to_stage("setup", setup.system())
        .add_startup_stage_after("setup", "prelude", SystemStage::parallel())
        .add_system(handle_esc.system())
        .add_startup_system_to_stage("prelude", play_audio.system())
        .add_system(
            increment_score
                .system()
                .with_run_criteria(FixedTimestep::step(INCREMENT_SCORE_STEP)),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(UiPlugin)
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
    });
    commands.insert_resource(BGMusic {
        handle: asset_server.load(BG_MUSIC),
        channel: AudioChannel::new("music".to_string()),
    });
    commands.insert_resource(SoundEffects {
        plane_handle: asset_server.load(PLANE_SFX),
        channel: AudioChannel::new("sfx".to_string()),
    })
}

fn setup_window(mut windows: ResMut<Windows>) {
    let primary_window = windows.get_primary_mut().expect("No windows found");
    primary_window.set_resizable(false);
    primary_window.set_position(IVec2::new(SCREEN_X, SCREEN_Y));
}

fn handle_esc(keyboard_input: Res<Input<KeyCode>>, mut exit_writer: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit_writer.send(AppExit);
    }
}

fn play_audio(audio: Res<Audio>, bg_music: Res<BGMusic>, sfx: Res<SoundEffects>) {
    audio.set_volume_in_channel(1.5, &bg_music.channel);
    audio.set_volume_in_channel(0.25, &sfx.channel);
    audio.play_looped_in_channel(bg_music.handle.clone(), &bg_music.channel);
    audio.play_looped_in_channel(sfx.plane_handle.clone(), &sfx.channel);
}

fn increment_score(mut score: ResMut<Score>) {
    score.increase_by(1);
}
