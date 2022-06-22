use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;

pub struct ImageBank<'lt> {
    background: Texture<'lt>,
}

impl<'lt> ImageBank<'lt> {
    pub fn load<T>(tc: &'lt TextureCreator<T>) -> Self {
        let mut background = include_raw_image!("res/background.jpeg");
        let surface =
            Surface::from_data(&mut background, 480, 270, 1440, PixelFormatEnum::RGB24).unwrap();

        Self {
            background: surface.as_texture(tc).unwrap(),
        }
    }

    pub fn get_background(&self) -> &Texture<'lt> {
        &self.background
    }
}
