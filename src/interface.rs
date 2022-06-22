use crate::pos::Direction;

#[derive(Debug)]
pub enum ClientMessage {
    ConnectionEnded {},
    EmptyCommand {},
    UnknownCommand { command: String },
    InvalidArguments { problem: String },
    EndGame {},
    GetMap {},
    Move { direction: Direction },
}

pub enum ServerMessage {
    Error { message: String },
    ValidMove {},
    MapResponse { map: String },
    EndGame {},
}

impl ClientMessage {
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        let msg = String::from_utf8_lossy(bytes);
        let msg_split: Vec<&str> = msg.split_whitespace().collect();

        if msg_split.len() > 0 {
            match msg_split[0] {
                "END" => {
                    if msg_split.len() == 1 {
                        Self::EndGame {}
                    } else {
                        Self::too_many_arguments("END")
                    }
                }
                "MAP" => {
                    if msg_split.len() == 1 {
                        Self::GetMap {}
                    } else {
                        Self::too_many_arguments("MAP")
                    }
                }
                "MOVE" => {
                    if msg_split.len() == 2 {
                        match msg_split[1] {
                            "DOWN" => Self::Move {
                                direction: Direction::DOWN,
                            },
                            "LEFT" => Self::Move {
                                direction: Direction::LEFT,
                            },
                            "RIGHT" => Self::Move {
                                direction: Direction::RIGHT,
                            },
                            "UP" => Self::Move {
                                direction: Direction::UP,
                            },
                            invalid => Self::InvalidArguments {
                                problem: format!("direction « {} » invalide", invalid),
                            },
                        }
                    } else if msg_split.len() > 2 {
                        Self::too_many_arguments("MOVE")
                    } else {
                        Self::not_enough_arguments("MOVE")
                    }
                }
                cmd => Self::UnknownCommand {
                    command: cmd.to_string(),
                },
            }
        } else {
            Self::EmptyCommand {}
        }
    }

    pub fn not_enough_arguments(command: &str) -> Self {
        Self::InvalidArguments {
            problem: format!("arguments manquants pour la commande « {} »", command),
        }
    }

    pub fn too_many_arguments(command: &str) -> Self {
        Self::InvalidArguments {
            problem: format!("trop d'arguments pour la commande « {} »", command),
        }
    }
}

impl ServerMessage {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Error { message } => format!("NOK {}", message),
            Self::ValidMove {} => "OK".to_string(),
            Self::MapResponse { map } => map,
            Self::EndGame {} => "END".to_string(),
        }
        .into_bytes()
    }
}
