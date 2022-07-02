use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;

pub enum TextRenderingFormat {
    Shaded(Color, Color),
    Blended(Color),
}

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

    pub fn render(&self, text: &str, format: TextRenderingFormat) -> RenderedTextResult {
        let partial = self.font.render(text);

        let surface = match format {
            TextRenderingFormat::Shaded(fg, bg) => partial.shaded(fg, bg),
            TextRenderingFormat::Blended(fg) => partial.blended(fg),
        }
        .unwrap();

        RenderedTextResult {
            texture: self
                .texture_creator
                .create_texture_from_surface(&surface)
                .unwrap(),
            w: surface.width(),
            h: surface.height(),
        }
    }
}

pub struct RenderedTextResult<'lt> {
    texture: Texture<'lt>,
    w: u32,
    h: u32,
}

impl RenderedTextResult<'_> {
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }
}
