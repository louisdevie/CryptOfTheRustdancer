use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;

pub struct TextRenderer<'lt, T> {
    font: Font<'lt, 'lt>,
    texture_creator: &'lt TextureCreator<T>,
}

impl<'lt, T> TextRenderer<'lt, T> {
    pub fn new(font: Font<'lt, 'lt>, texture_creator: &'lt TextureCreator<T>) -> Self {
        Self {
            font,
            texture_creator,
        }
    }

    pub fn render(&self, text: &str, foreground: Color, background: Color) -> RenderedTextResult {
        let surface = self
            .font
            .render(text)
            .shaded(foreground, background)
            .unwrap();

        RenderedTextResult(
            self.texture_creator
                .create_texture_from_surface(&surface)
                .unwrap(),
            surface.width(),
            surface.height(),
        )
    }
}

pub struct RenderedTextResult<'lt>(Texture<'lt>, u32, u32);

impl RenderedTextResult<'_> {
    pub fn texture(&self) -> &Texture {
        &self.0
    }

    pub fn width(&self) -> u32 {
        self.1
    }

    pub fn height(&self) -> u32 {
        self.2
    }
}
