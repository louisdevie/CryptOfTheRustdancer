use sdl2::mixer::AUDIO_S16LSB;
use sdl2::mixer::DEFAULT_CHANNELS;
use sdl2::mixer::{open_audio, Music};

pub struct Sounds<'lt> {
    menu_music: Music<'lt>,
    in_game_music: Music<'lt>,
}

impl Sounds<'_> {
    pub fn load() -> Self {
        open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 512).unwrap();
        Self {
            menu_music: Music::from_static_bytes(include_bytes!("../../res/menu.mp3")).unwrap(),
            in_game_music: Music::from_static_bytes(include_bytes!("../../res/level.mp3")).unwrap(),
        }
    }

    pub fn play_menu_music(&self) {
        self.menu_music.play(-1).unwrap();
    }

    pub fn play_in_game_music(&self) {
        self.in_game_music.play(-1).unwrap();
    }
}
