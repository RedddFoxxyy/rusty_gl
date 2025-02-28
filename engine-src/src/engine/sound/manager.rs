/// # The Sound Management Module for the Terra Graphics Engine!
/// > Note: Still under heavy developement!!! Unsafe to use in production!
use std::collections::HashMap;

use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend,
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

struct SoundManager {
    manager: AudioManager,
    sounds: HashMap<String, StaticSoundData>,
}
impl SoundManager {
    pub fn load_sound(&mut self, name: String, filepath: String) {
        let maybe_sound_data = StaticSoundData::from_file(filepath);
        if let Err(e) = maybe_sound_data {
            println!("Failed to load the sound file: {}", e);
            return;
        }
        let sound_data = maybe_sound_data.unwrap();

        self.sounds.insert(name, sound_data.clone());
    }

    // TODO: I think the manager.play function only plays the sound once,
    // instead we need this sound to keep playing in a loop
    // add a new func called play_sound_loop( can use a good name )
    // that plays the sound in a loop.
    //
    // If manager.play() does loop the audio then no need of that.
    // > @yourpeepee
    pub fn play_sound(&mut self, name: String) {
        let maybe_sound_data = self.sounds.get(&name);
        if maybe_sound_data.is_none() {
            println!("Err in playing, sound file of this name is not loaded.");
            return;
        }

        let sound_data = maybe_sound_data.unwrap();
        let play_err = self.manager.play(sound_data.clone());

        if let Err(e) = play_err {
            println!("Failed to play the sound in sound manager: {}", e);
        }
    }

    pub fn stop_sound(&self, name: String) {
        let maybe_sound_data = self.sounds.get(&name);
        if maybe_sound_data.is_none() {
            println!("Cannot stop the sound!!!");
        }

        let sound_data = maybe_sound_data.unwrap();
    }
}
