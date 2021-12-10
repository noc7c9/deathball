use kira::{
    instance::{handle::InstanceHandle, InstanceState},
    manager::AudioManager as KiraAudioManager,
    sound::{handle::SoundHandle, Sound},
};
use macroquad::prelude::*;

use super::Assets;

const MAX_HIT_SFX_PLAYING: usize = 4;

pub struct AudioManager {
    _manager: KiraAudioManager,
    hit_sfx: SoundEffects,
}

impl AudioManager {
    pub fn new(assets: &Assets) -> Self {
        let mut manager = KiraAudioManager::new(Default::default()).unwrap();
        let hit_sfx = SoundEffects::new(&mut manager, &assets.smack, MAX_HIT_SFX_PLAYING);
        Self {
            // keep ref to make sure manager isn't dropped
            _manager: manager,
            hit_sfx,
        }
    }

    pub fn play_hit_sfx(&mut self) {
        self.hit_sfx.play();
    }
}

struct SoundEffects {
    sounds: Vec<SoundHandle>,
    playing: Vec<InstanceHandle>,
}

impl SoundEffects {
    fn new(manager: &mut KiraAudioManager, sound_bytes: &[Vec<u8>], max_playing: usize) -> Self {
        let sounds = sound_bytes
            .iter()
            .map(|bytes| {
                let cursor = std::io::Cursor::new(bytes);
                let sound = Sound::from_ogg_reader(cursor, Default::default()).unwrap();
                manager.add_sound(sound).unwrap()
            })
            .collect();

        Self {
            sounds,
            playing: Vec::with_capacity(max_playing),
        }
    }

    fn play(&mut self) {
        self.playing
            .retain(|handle| handle.state() != InstanceState::Stopped);

        if self.playing.len() < self.playing.capacity() {
            let idx = rand::gen_range(0, self.sounds.len());
            let handle = self.sounds[idx].play(Default::default()).unwrap();
            self.playing.push(handle);
        }
    }
}
