use sfml::audio::Music as SfmlSound;
use sfml::audio::SoundBuffer;

pub struct Sound {
    sound: SfmlSound,
}

impl Sound {
    pub fn new(s: &str) -> Sound {
        Sound {
            sound: SfmlSound::new_from_file(s).unwrap(),
        }
    }

    pub fn repeat(&mut self) -> &mut Self {
        self.sound.set_loop(true);
        self
    }

    pub fn play(&mut self) -> &mut Self {
        self.sound.play();
        self
    }

    pub fn stop(&mut self) -> &mut Self {
        self.sound.stop();
        self
    }

    pub fn pause(&mut self) -> &mut Self {
        self.sound.pause();
        self
    }

    pub fn volume(&mut self, vol: f32) -> &mut Self {
        self.sound.set_volume(vol);
        self
    }
}
