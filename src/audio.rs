use kira::{
    instance::{
        handle::InstanceHandle, InstanceLoopStart, InstanceSettings, InstanceState,
        StopInstanceSettings,
    },
    manager::AudioManager as KiraAudioManager,
    parameter::tween::Tween,
    sound::{handle::SoundHandle, Sound as KiraSound},
};
use macroquad::prelude::*;

use super::Assets;

const BGM_VOLUME: f64 = 0.5;
const SFX_VOLUME: f64 = 0.5;
const BGM_FADE_IN_TIME: f64 = 2.0;
const BGM_FADE_OUT_TIME: f64 = 1.0;

const MAX_HIT_SFX_PLAYING: usize = 4;
const MAX_KILLED_SFX_PLAYING: usize = 6;

pub struct AudioManager {
    _manager: KiraAudioManager,
    pub bgm: BackgroundMusic,
    pub hit_sfx: SoundEffects,
    pub killed_sfx: SoundEffects,
}

impl AudioManager {
    pub fn new(assets: &mut Assets) -> Self {
        let mut manager = KiraAudioManager::new(Default::default()).unwrap();
        let bgm = BackgroundMusic::new(&mut manager, assets);
        let hit_sfx = SoundEffects::new(&mut manager, &mut assets.smack, MAX_HIT_SFX_PLAYING);
        let killed_sfx =
            SoundEffects::new(&mut manager, &mut assets.explode, MAX_KILLED_SFX_PLAYING);
        Self {
            // keep ref to make sure manager isn't dropped
            _manager: manager,
            bgm,
            hit_sfx,
            killed_sfx,
        }
    }
}

// used by the assets module to decode and store sound data before it's needed by this module
pub struct Sound(Option<KiraSound>);

impl Sound {
    pub fn new(bytes: Vec<u8>) -> Self {
        let cursor = std::io::Cursor::new(bytes);
        let sound = KiraSound::from_ogg_reader(cursor, Default::default()).unwrap();
        Sound(Some(sound))
    }

    fn add(&mut self, manager: &mut KiraAudioManager) -> SoundHandle {
        manager.add_sound(self.0.take().unwrap()).unwrap()
    }
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
    fn new(manager: &mut KiraAudioManager, assets: &mut Assets) -> Self {
        Self {
            giant_horse_deathball: assets.giant_horse_deathball.add(manager),
            meadow_meadow: assets.meadow_meadow.add(manager),
            send_it: assets.send_it.add(manager),
            space: assets.space.add(manager),
            take_me_home: assets.take_me_home.add(manager),
            playing: None,
        }
    }

    fn stop(&mut self) {
        if let Some(mut instance) = self.playing.take() {
            instance
                .stop(StopInstanceSettings {
                    fade_tween: Self::linear_tween(BGM_FADE_OUT_TIME),
                })
                .expect("Failed to stop background music");
        }
    }

    pub fn play(&mut self, track: bgm::Track) {
        if crate::debug::DISABLE_BGM {
            return;
        }

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
                volume: BGM_VOLUME.into(),
                fade_in_tween: Self::linear_tween(BGM_FADE_IN_TIME),
                loop_start: InstanceLoopStart::Custom(0.),
                ..Default::default()
            })
            .ok();
    }

    fn linear_tween(time: f64) -> Option<Tween> {
        Some(Tween::linear(time))
    }
}

pub struct SoundEffects {
    sounds: Vec<SoundHandle>,
    playing: Vec<InstanceHandle>,
}

impl SoundEffects {
    fn new(manager: &mut KiraAudioManager, sounds: &mut [Sound], max_playing: usize) -> Self {
        let sounds = sounds.iter_mut().map(|sound| sound.add(manager)).collect();

        Self {
            sounds,
            playing: Vec::with_capacity(max_playing),
        }
    }

    pub fn play(&mut self) {
        if crate::debug::DISABLE_SFX {
            return;
        }

        self.playing
            .retain(|handle| handle.state() != InstanceState::Stopped);

        if self.playing.len() < self.playing.capacity() {
            let idx = rand::gen_range(0, self.sounds.len());
            let handle = self.sounds[idx]
                .play(InstanceSettings {
                    volume: SFX_VOLUME.into(),
                    ..Default::default()
                })
                .unwrap();
            self.playing.push(handle);
        }
    }
}
