use kira::{
    instance::{
        handle::InstanceHandle, InstanceLoopStart, InstanceSettings, InstanceState,
        StopInstanceSettings,
    },
    manager::AudioManager as KiraAudioManager,
    parameter::tween::Tween,
    sound::{handle::SoundHandle, Sound},
};
use macroquad::prelude::*;

use super::Assets;

const FADE_TIME: f64 = 2.0;
const MAX_HIT_SFX_PLAYING: usize = 4;
const MAX_KILLED_SFX_PLAYING: usize = 6;

pub struct AudioManager {
    _manager: KiraAudioManager,
    pub bgm: BackgroundMusic,
    hit_sfx: SoundEffects,
    killed_sfx: SoundEffects,
}

impl AudioManager {
    pub fn new(assets: &Assets) -> Self {
        let mut manager = KiraAudioManager::new(Default::default()).unwrap();
        let bgm = BackgroundMusic::new(&mut manager, assets);
        let hit_sfx = SoundEffects::new(&mut manager, &assets.smack, MAX_HIT_SFX_PLAYING);
        let killed_sfx = SoundEffects::new(&mut manager, &assets.explode, MAX_KILLED_SFX_PLAYING);
        Self {
            // keep ref to make sure manager isn't dropped
            _manager: manager,
            bgm,
            hit_sfx,
            killed_sfx,
        }
    }

    pub fn play_hit_sfx(&mut self) {
        self.hit_sfx.play();
    }

    pub fn play_killed_sfx(&mut self) {
        self.killed_sfx.play();
    }
}

fn add_sound(manager: &mut KiraAudioManager, bytes: &[u8]) -> SoundHandle {
    let cursor = std::io::Cursor::new(bytes);
    let sound = Sound::from_ogg_reader(cursor, Default::default()).unwrap();
    manager.add_sound(sound).unwrap()
}

pub mod bgm {
    #[derive(Clone, Copy)]
    pub enum Track {
        GiantHorseDeathball,
        MeadowMeadow,
        SendIt,
        Space,
        TakeMeHome,
    }

    pub use Track::*;
}

pub struct BackgroundMusic {
    giant_horse_deathball: SoundHandle,
    meadow_meadow: SoundHandle,
    send_it: SoundHandle,
    space: SoundHandle,
    take_me_home: SoundHandle,
    playing: Option<InstanceHandle>,
}

impl BackgroundMusic {
    fn new(manager: &mut KiraAudioManager, assets: &Assets) -> Self {
        Self {
            giant_horse_deathball: add_sound(manager, &assets.giant_horse_deathball),
            meadow_meadow: add_sound(manager, &assets.meadow_meadow),
            send_it: add_sound(manager, &assets.send_it),
            space: add_sound(manager, &assets.space),
            take_me_home: add_sound(manager, &assets.take_me_home),
            playing: None,
        }
    }

    pub fn stop(&mut self) {
        if let Some(mut instance) = self.playing.take() {
            instance
                .stop(StopInstanceSettings {
                    fade_tween: Self::linear_tween(),
                })
                .expect("Failed to stop background music");
        }
    }

    pub fn play(&mut self, track: bgm::Track) {
        self.stop();

        let sound = match track {
            bgm::GiantHorseDeathball => &mut self.giant_horse_deathball,
            bgm::MeadowMeadow => &mut self.meadow_meadow,
            bgm::SendIt => &mut self.send_it,
            bgm::Space => &mut self.space,
            bgm::TakeMeHome => &mut self.take_me_home,
        };

        self.playing = sound
            .play(InstanceSettings {
                fade_in_tween: Self::linear_tween(),
                loop_start: InstanceLoopStart::Custom(0.),
                ..Default::default()
            })
            .ok();
    }

    fn linear_tween() -> Option<Tween> {
        Some(Tween::linear(FADE_TIME))
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
            .map(|bytes| add_sound(manager, bytes))
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
