use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioSource};

/// A global store of all materials used in the game
pub struct Materials {
    pub plane_material: Handle<ColorMaterial>,
    pub red_enemy_material: Handle<ColorMaterial>,
    pub yellow_enemy_material: Handle<ColorMaterial>,
}

pub struct Fonts {
    pub main_font: Handle<Font>,
}

pub struct Score(pub u32);

impl Score {
    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }

    pub fn increase_by(&mut self, value: u32) {
        self.set(self.0 + value);
    }
}

pub struct BGMusic {
    pub handle: Handle<AudioSource>,
    pub channel: AudioChannel,
}

pub struct SoundEffects {
    pub plane_handle: Handle<AudioSource>,
    pub channel: AudioChannel,
}
