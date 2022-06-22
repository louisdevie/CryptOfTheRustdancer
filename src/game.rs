use crate::interface::{ClientMessage, ServerMessage};
use crate::map::Map;
use sdl2::event::Event;

pub struct Game {
    map: Map,
}

impl Game {
    pub fn new() -> Self {
        Self { map: Map::empty() }
    }

    pub fn reset(&mut self, seed: u32) {
        self.map = Map::generate(seed);
    }

    pub fn handle_event(&mut self, _event: Event) {}

    pub fn react_to_message(&mut self, message: ClientMessage) -> ServerMessage {
        match message {
            ClientMessage::EmptyCommand {} => ServerMessage::Error {
                message: "commande vide".to_string(),
            },
            ClientMessage::UnknownCommand { command } => ServerMessage::Error {
                message: format!("commande « {} » inconnue", command),
            },
            ClientMessage::InvalidArguments { problem } => {
                ServerMessage::Error { message: problem }
            }
            ClientMessage::EndGame {} => ServerMessage::EndGame {},

            ClientMessage::GetMap {} => ServerMessage::MapResponse {
                map: self.map.repr(),
            },
            ClientMessage::Move { direction } => {
                self.map.move_player(direction);
                ServerMessage::ValidMove {}
            }
            ClientMessage::ConnectionEnded {} => ServerMessage::Error {
                message: "internal error : match arm should not be reachable".to_string(),
            },
        }
    }

    pub fn draw(&self) {}
}
