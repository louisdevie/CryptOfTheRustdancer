use crate::game::pos::Direction;

#[derive(Debug)]
pub enum ClientMessage {
    ConnectionEnded,
    EmptyCommand,
    UnknownCommand(String),
    InvalidArguments(String),
    EndGame,
    GetMap,
    Move(Direction),
}

pub enum ServerMessage {
    EndConnection,
    Error(String),
    ValidMove,
    MapResponse(String),
    EndGame,
}

impl ClientMessage {
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        let msg = String::from_utf8_lossy(bytes);
        let msg_split: Vec<&str> = msg.split_whitespace().collect();

        if msg_split.len() > 0 {
            match msg_split[0] {
                "END" => {
                    if msg_split.len() == 1 {
                        Self::EndGame
                    } else {
                        Self::too_many_arguments("END")
                    }
                }
                "MAP" => {
                    if msg_split.len() == 1 {
                        Self::GetMap
                    } else {
                        Self::too_many_arguments("MAP")
                    }
                }
                "MOVE" => {
                    if msg_split.len() == 2 {
                        match msg_split[1] {
                            "DOWN" => Self::Move(Direction::DOWN),
                            "LEFT" => Self::Move(Direction::LEFT),
                            "RIGHT" => Self::Move(Direction::RIGHT),
                            "UP" => Self::Move(Direction::UP),
                            invalid => Self::InvalidArguments(format!(
                                "direction « {} » invalide",
                                invalid
                            )),
                        }
                    } else if msg_split.len() > 2 {
                        Self::too_many_arguments("MOVE")
                    } else {
                        Self::not_enough_arguments("MOVE")
                    }
                }
                cmd => Self::UnknownCommand(cmd.to_string()),
            }
        } else {
            Self::EmptyCommand
        }
    }

    pub fn not_enough_arguments(command: &str) -> Self {
        Self::InvalidArguments(format!(
            "arguments manquants pour la commande « {} »",
            command
        ))
    }

    pub fn too_many_arguments(command: &str) -> Self {
        Self::InvalidArguments(format!("trop d'arguments pour la commande « {} »", command))
    }
}

impl ServerMessage {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Error(message) => format!("NOK {}", message),
            Self::ValidMove => "OK".to_string(),
            Self::MapResponse(map) => map,
            Self::EndGame => "END".to_string(),
            Self::EndConnection => "".to_string(),
        }
        .into_bytes()
    }
}
