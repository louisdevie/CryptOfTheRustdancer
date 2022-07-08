use crate::interface::{ClientMessage, ServerMessage};
use crate::resource::image::Images;
use crate::resource::text::TextRenderer;
use crate::resource::text::TextRenderingFormat::{Blended, Shaded};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

pub mod map;
pub mod pos;

use map::{Map, Tile};
use pos::{Direction, Pos};

enum Action {
    Nothing,
    PlayerMovement(Direction),
    DigWall(Pos),
}

#[derive(PartialEq)]
enum State {
    PLAYING,
    STOPPED,
    WON,
}

pub struct Game {
    map: Map,
    diamonds_left: usize,
    animations_step_1: u8, // pour les animations des lutins
    animations_step_2: u8, // pour les animations de mouvement
    timer_end: u8,
    response: Option<ServerMessage>,
    reaction: Action,
    player_sprite_direction: Direction,
    state: State,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: Map::empty(),
            diamonds_left: 0,
            animations_step_1: 0,
            animations_step_2: 0,
            response: None,
            reaction: Action::Nothing,
            player_sprite_direction: Direction::RIGHT,
            state: State::STOPPED,
            timer_end: 0,
        }
    }

    pub fn reset(&mut self, seed: u32) {
        self.map = Map::generate(seed);
        self.diamonds_left = self.map.diamonds_count();

        self.animations_step_1 = 7;
        self.animations_step_2 = 0;
        self.timer_end = 6;

        self.response = None;
        self.reaction = Action::Nothing;
        self.state = State::PLAYING;

        self.player_sprite_direction = Direction::RIGHT;
    }

    pub fn tick(&mut self) -> bool {
        self.animations_step_1 += 1;
        self.animations_step_1 %= 8;

        match self.reaction {
            Action::Nothing => {
                if self.state != State::PLAYING {
                    if self.timer_end == 0 {
                        self.response = Some(ServerMessage::EndConnection);
                        true
                    } else {
                        self.timer_end -= 1;
                        false
                    }
                } else {
                    true
                }
            }
            Action::PlayerMovement(direction) => {
                self.animations_step_2 += 1;
                if self.animations_step_2 == 4 {
                    self.map.move_player(direction);
                    if self.diamonds_left == 0
                        && self.map.tile_at(self.map.player_pos()) == Some(Tile::EXIT)
                    {
                        self.state = State::WON;
                    }
                    self.map.pick_up_diamond();
                    self.diamonds_left = self.map.diamonds_count();
                    self.reaction = Action::Nothing;
                    self.animations_step_2 = 0;
                }
                false
            }
            Action::DigWall(position) => {
                self.animations_step_2 += 1;
                if self.animations_step_2 == 4 {
                    self.map.dig(position);
                    self.reaction = Action::Nothing;
                    self.animations_step_2 = 0;
                }
                false
            }
        }
    }

    pub fn handle_event(&mut self, ev: Event) {
        match ev {
            Event::KeyUp { keycode, .. } => match keycode {
                Some(Keycode::Escape) => {
                    if self.state != State::WON {
                        self.state = State::STOPPED
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn react_to_message(&mut self, message: ClientMessage) {
        if self.state == State::PLAYING {
            (self.response, self.reaction) = match message {
                ClientMessage::EmptyCommand => (
                    Some(ServerMessage::Error("commande vide".to_string())),
                    Action::Nothing,
                ),

                ClientMessage::UnknownCommand(command) => (
                    Some(ServerMessage::Error(format!(
                        "commande « {} » inconnue",
                        command
                    ))),
                    Action::Nothing,
                ),

                ClientMessage::InvalidArguments(problem) => {
                    (Some(ServerMessage::Error(problem)), Action::Nothing)
                }

                ClientMessage::EndGame => {
                    self.state = State::STOPPED;
                    (Some(ServerMessage::EndGame {}), Action::Nothing)
                }

                ClientMessage::GetMap => (
                    Some(ServerMessage::MapResponse(self.map.repr())),
                    Action::Nothing,
                ),

                ClientMessage::Move(direction) => {
                    let dest = self.map.player_pos().moved(direction);
                    match direction {
                        Direction::LEFT => self.player_sprite_direction = Direction::LEFT,
                        Direction::RIGHT => self.player_sprite_direction = Direction::RIGHT,
                        _ => {}
                    }
                    match self.map.tile_at(dest) {
                        Some(Tile::EMPTY) => (
                            Some(ServerMessage::ValidMove),
                            Action::PlayerMovement(direction),
                        ),
                        Some(Tile::WALL) => (
                            Some(ServerMessage::ValidMove),
                            Action::DigWall(self.map.player_pos().moved(direction)),
                        ),

                        Some(Tile::EXIT) => (
                            Some(ServerMessage::ValidMove),
                            Action::PlayerMovement(direction),
                        ),
                        Some(Tile::STONE) | Some(Tile::BORDER) | None => (
                            Some(ServerMessage::Error("mouvement invalide".to_string())),
                            Action::Nothing,
                        ),
                    }
                }
                ClientMessage::ConnectionEnded => (
                    Some(ServerMessage::Error(
                        "internal error : match arm should not be reachable".to_string(),
                    )),
                    Action::Nothing,
                ),
            }
        }
    }

    pub fn response(&mut self) -> Option<ServerMessage> {
        match self.reaction {
            Action::Nothing => match &mut self.response {
                None => None,
                response => std::mem::replace(response, None),
            },
            _ => None,
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
        let offset_x = self.map.player_pos().x as i32 * -72 + 684
            - animation_offset_x(&self.reaction, self.animations_step_2);
        let offset_y = self.map.player_pos().y as i32 * -72 + 333
            - animation_offset_y(&self.reaction, self.animations_step_2);

        for y in 0..37 {
            for x in 0..37 {
                canvas
                    .copy(
                        match self.map.tile_at(Pos::new(x, y)).unwrap() {
                            Tile::EMPTY => {
                                if (x + y + (self.animations_step_1 as u32 / 4)) % 2 == 0 {
                                    images.floor_green()
                                } else {
                                    images.floor_purple()
                                }
                            }
                            Tile::WALL => images.wall(),
                            Tile::STONE => images.stone(),
                            Tile::BORDER => images.border(),
                            Tile::EXIT => {
                                if self.diamonds_left == 0 {
                                    images.exit()
                                } else {
                                    images.exit_locked()
                                }
                            }
                        },
                        None,
                        Rect::new(
                            (x * 72) as i32 + offset_x,
                            (y * 72) as i32 + offset_y,
                            72,
                            144,
                        ),
                    )
                    .unwrap();
                for d in self.map.diamonds_pos() {
                    if d == (x, y) {
                        canvas
                            .copy(
                                images.diamond(),
                                None,
                                Rect::new(
                                    (x * 72) as i32 + offset_x,
                                    (y * 72) as i32 + offset_y,
                                    72,
                                    144,
                                ),
                            )
                            .unwrap();
                    }
                }

                match self.reaction {
                    Action::DigWall(position) => {
                        if position == (x, y) {
                            canvas
                                .copy(
                                    images.shovel(),
                                    None,
                                    Rect::new(
                                        (x * 72) as i32 + offset_x,
                                        (y * 72) as i32 + offset_y,
                                        72,
                                        144,
                                    ),
                                )
                                .unwrap();
                        }
                    }
                    _ => {}
                }

                if match self.reaction {
                    Action::PlayerMovement(Direction::RIGHT) => {
                        if x == 0 {
                            false
                        } else {
                            self.map.player_pos() == (x - 1, y)
                        }
                    }
                    Action::PlayerMovement(Direction::DOWN) => {
                        if y == 0 {
                            false
                        } else {
                            self.map.player_pos() == (x, y - 1)
                        }
                    }
                    _ => self.map.player_pos() == (x, y),
                } {
                    canvas
                        .copy_ex(
                            match self.animations_step_1 {
                                0 | 4 => images.cadence_1(),
                                1 | 5 => images.cadence_2(),
                                2 | 6 => images.cadence_3(),
                                _ => images.cadence_4(),
                            },
                            None,
                            Rect::new(
                                684,
                                333 - animation_camera_offset_y(
                                    &self.reaction,
                                    self.animations_step_2,
                                ),
                                72,
                                144,
                            ),
                            0.0,
                            None,
                            self.player_sprite_direction == Direction::LEFT,
                            false,
                        )
                        .unwrap();
                }
            }
        }

        canvas.fill_rect(Rect::new(1356, 63, 216, 33)).unwrap();
        let diamond_text = text_renderer.render(
            &format!("x{}", 10 - self.diamonds_left),
            Blended(Color::RGB(255, 255, 255)),
        );
        canvas
            .copy(
                diamond_text.texture(),
                None,
                Rect::new(1386, 66, diamond_text.width(), diamond_text.height()),
            )
            .unwrap();
        canvas
            .copy(images.diamond_icon(), None, Rect::new(1332, 60, 51, 39))
            .unwrap();

        match self.state {
            State::PLAYING => {}
            State::STOPPED => draw_message("Partie interrompue", text_renderer, canvas),
            State::WON => draw_message("Niveau terminé", text_renderer, canvas),
        }
    }
}

fn draw_message<T, U>(msg: &str, tr: &TextRenderer<U>, c: &mut Canvas<T>)
where
    T: RenderTarget,
{
    let message = tr.render(msg, Shaded(Color::RGB(255, 255, 255), Color::RGB(0, 0, 0)));
    c.fill_rect(Rect::new(
        690 - message.width() as i32 / 2,
        390 - message.height() as i32 / 2,
        message.width() + 60,
        message.height() + 30,
    ))
    .unwrap();
    c.copy(
        message.texture(),
        None,
        Rect::new(
            720 - message.width() as i32 / 2,
            405 - message.height() as i32 / 2,
            message.width(),
            message.height(),
        ),
    )
    .unwrap();
}

fn animation_offset_x(action: &Action, step: u8) -> i32 {
    match action {
        Action::PlayerMovement(direction) => match direction {
            Direction::UP | Direction::DOWN => 0,
            Direction::RIGHT => match step {
                0 => 0,
                1 => 9,
                2 => 36,
                3 => 63,
                _ => 72,
            },
            Direction::LEFT => match step {
                0 => 0,
                1 => -9,
                2 => -36,
                3 => -63,
                _ => -72,
            },
        },
        Action::Nothing | Action::DigWall { .. } => 0,
    }
}

fn animation_offset_y(action: &Action, step: u8) -> i32 {
    match action {
        Action::PlayerMovement(direction) => match direction {
            Direction::RIGHT | Direction::LEFT => 0,
            Direction::DOWN => match step {
                0 => 0,
                1 => 3,
                2 => 12,
                3 => 39,
                _ => 72,
            },
            Direction::UP => match step {
                0 => 0,
                1 => -33,
                2 => -60,
                3 => -69,
                _ => -72,
            },
        },
        Action::Nothing | Action::DigWall { .. } => 0,
    }
}

fn animation_camera_offset_y(action: &Action, step: u8) -> i32 {
    match action {
        Action::PlayerMovement(direction) => match direction {
            Direction::RIGHT | Direction::LEFT => match step {
                0 => 0,
                1 => 6,
                2 => 9,
                3 => 6,
                _ => 0,
            },
            Direction::DOWN | Direction::UP => 0,
        },
        Action::Nothing | Action::DigWall { .. } => 0,
    }
}
