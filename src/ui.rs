use bevy::prelude::*;

use crate::shared::{Fonts, Score};

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
    if !score.is_changed() {
        return;
    }
    for mut text in label.iter_mut() {
        text.sections[0].value = format!("{}", score.0);
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage("prelude", spawn_ui.system())
            .add_system(update_score_label.system());
    }
}
