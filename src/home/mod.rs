use crate::resource::image::Images;
use crate::resource::text::TextRenderer;
use crate::resource::text::TextRenderingFormat::Shaded;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use std::str::FromStr;

pub struct Home {
    seed: u32,
    editing: bool,
    input_text: String,
}

impl Home {
    pub fn new() -> Self {
        Self {
            editing: false,
            input_text: "0".to_string(),
            seed: 0,
        }
    }

    pub fn ready(&self) -> bool {
        !self.editing
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }

    pub fn handle_event(&mut self, ev: Event) {
        match ev {
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if mouse_btn == MouseButton::Left && is_inside((x, y), (0, 730, 720, 810)) {
                    self.editing = true;
                }
            }
            Event::TextInput { text, .. } => {
                if self.editing && filter(&text) {
                    self.input_text.push_str(&text)
                }
            }
            Event::KeyDown { keycode, .. } => match keycode {
                Some(Keycode::Return) => {
                    self.editing = false;
                    match u32::from_str(&self.input_text) {
                        Ok(seed) => self.seed = seed,
                        Err(_) => self.seed = 0,
                    }
                    self.input_text = self.seed.to_string();
                }
                Some(Keycode::Backspace) => {
                    self.input_text.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn draw<T, U>(
        &self,
        canvas: &mut Canvas<T>,
        images: &Images,
        text_renderer: &TextRenderer<U>,
    ) where
        T: RenderTarget,
    {
        canvas.copy(images.background(), None, None).unwrap();

        let seed_text = text_renderer.render(
            &format!(
                "Seed: {}{}",
                self.input_text,
                if self.editing { "_" } else { "" }
            ),
            Shaded(Color::RGB(255, 255, 255), Color::RGB(0, 0, 0)),
        );
        let status_text = text_renderer.render(
            if self.editing {
                "Appuyez sur [ENTRÉE] pour valider"
            } else {
                "En attente du client..."
            },
            Shaded(Color::RGB(255, 255, 255), Color::RGB(0, 0, 0)),
        );

        canvas
            .copy(
                seed_text.texture(),
                None,
                Rect::new(16, 770, seed_text.width(), seed_text.height()),
            )
            .unwrap();
        canvas
            .copy(
                status_text.texture(),
                None,
                Rect::new(
                    1424 - status_text.width() as i32,
                    770,
                    status_text.width(),
                    status_text.height(),
                ),
            )
            .unwrap();
    }
}

fn is_inside(point: (i32, i32), bounds: (i32, i32, i32, i32)) -> bool {
    bounds.0 <= point.0 && point.0 <= bounds.2 && bounds.1 <= point.1 && point.1 <= bounds.3
}

fn filter(string: &str) -> bool {
    for c in string.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
