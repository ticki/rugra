use sfml::audio::Music as SfmlSound;

/// Sound
pub struct Sound {
    sound: SfmlSound,
}

impl Sound {
    /// Load a sound from a file
    pub fn load(s: &str) -> Sound {
        Sound {
            sound: SfmlSound::new_from_file(s).unwrap(),
        }
    }

    /// Repeat the sound
    pub fn repeat(&mut self) -> &mut Self {
        self.sound.set_loop(true);
        self
    }

    /// Play the sound
    pub fn play(&mut self) -> &mut Self {
        self.sound.play();
        self
    }

    /// Stop the sound
    pub fn stop(&mut self) -> &mut Self {
        self.sound.stop();
        self
    }

    /// Pause the sound
    pub fn pause(&mut self) -> &mut Self {
        self.sound.pause();
        self
    }

    /// Set the volume
    pub fn volume(&mut self, vol: f32) -> &mut Self {
        self.sound.set_volume(vol);
        self
    }
}
