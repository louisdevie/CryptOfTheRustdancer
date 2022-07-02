use sdl2::mixer::Music;

pub struct Sounds<'lt> {
    menu_music: Music<'lt>,
    in_game_music: Music<'lt>,
}

impl Sounds<'_> {
    pub fn load() -> Self {
        Self {
            menu_music: Music::from_static_bytes(include_bytes!("../res/menu.mp3")).unwrap(),
            in_game_music: Music::from_static_bytes(include_bytes!("../res/level.mp3")).unwrap(),
        }
    }
}
