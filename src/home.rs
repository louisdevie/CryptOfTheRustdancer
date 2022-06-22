use crate::image::ImageBank;
use crate::text::TextRenderer;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

pub struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_event(&mut self, _: Event) {}

    pub fn draw<T, U>(
        &self,
        canvas: &mut Canvas<T>,
        images: &ImageBank,
        text_renderer: &TextRenderer<U>,
    ) where
        T: RenderTarget,
    {
        canvas.copy(images.get_background(), None, None).unwrap();

        let text = text_renderer.render("Seed : 0", Color::RGB(255, 255, 255), Color::RGB(0, 0, 0));
        canvas
            .copy(
                text.texture(),
                None,
                Rect::new(16, 770, text.width(), text.height()),
            )
            .unwrap();
    }
}
