use super::pos::{Direction, Pos};
use oorandom::Rand32;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Tile {
    EMPTY,
    WALL,
    STONE,
    BORDER,
    EXIT,
}

struct Player {
    pos: Pos,
}

struct Diamond {
    pos: Pos,
}

pub struct Map {
    terrain: HashMap<Pos, Tile>,
    player: Player,
    diamonds: Vec<Diamond>,
}

impl Map {
    pub fn generate(seed: u32) -> Self {
        let mut rng = Rand32::new(seed.into());
        let mut terrain = HashMap::new();

        for x in 1..36 {
            for y in 1..36 {
                terrain.insert(Pos::new(x, y), Tile::WALL);
            }
        }

        for x in 0..36 {
            terrain.insert(Pos::new(x, 0), Tile::BORDER);
        }
        for y in 0..36 {
            terrain.insert(Pos::new(36, y), Tile::BORDER);
        }
        for x in (1..37).rev() {
            terrain.insert(Pos::new(x, 36), Tile::BORDER);
        }
        for y in (1..37).rev() {
            terrain.insert(Pos::new(0, y), Tile::BORDER);
        }

        for _ in 0..rng.rand_range(10..40) {
            terrain.insert(
                Pos::new(rng.rand_range(2..35), rng.rand_range(2..35)),
                Tile::STONE,
            );
        }

        let room_count = rng.rand_range(4..7);
        let mut rooms_center = Vec::with_capacity(7);
        for _ in 0..room_count {
            let top = rng.rand_range(2..25);
            let left = rng.rand_range(2..25);

            rooms_center.push((left + 5, top + 5));

            for x in 1..10 {
                for y in 1..10 {
                    terrain.insert(Pos::new(left + x, top + y), Tile::EMPTY);
                }
            }

            if rng.rand_float() < 0.5 {
                for y in 2..9 {
                    terrain.insert(Pos::new(left, top + y), Tile::EMPTY);
                    terrain.insert(Pos::new(left + 10, top + y), Tile::EMPTY);
                }
            }

            if rng.rand_float() < 0.5 {
                for x in 2..9 {
                    terrain.insert(Pos::new(left + x, top), Tile::EMPTY);
                    terrain.insert(Pos::new(left + x, top + 10), Tile::EMPTY);
                }
            }
        }

        for c in &rooms_center {
            terrain.insert(Pos::from(*c), Tile::BORDER);
        }

        for i in 0..room_count {
            for j in i + 1..room_count {
                let i = i as usize;
                let j = j as usize;
                if rng.rand_float() < 0.5 {
                    if rooms_center[i].0 < rooms_center[j].0 {
                        for x in rooms_center[i].0..rooms_center[j].0 + 1 {
                            terrain.insert(Pos::new(x, rooms_center[i].1), Tile::EMPTY);
                        }
                    } else {
                        for x in rooms_center[j].0..rooms_center[i].0 + 1 {
                            terrain.insert(Pos::new(x, rooms_center[i].1), Tile::EMPTY);
                        }
                    }
                    if rooms_center[i].1 < rooms_center[j].1 {
                        for y in rooms_center[i].1..rooms_center[j].1 + 1 {
                            terrain.insert(Pos::new(rooms_center[j].0, y), Tile::EMPTY);
                        }
                    } else {
                        for y in rooms_center[j].1..rooms_center[i].1 + 1 {
                            terrain.insert(Pos::new(rooms_center[j].0, y), Tile::EMPTY);
                        }
                    }
                }
            }
        }

        let mut diamonds = Vec::new();

        for _ in 0..10 {
            let d = Diamond::new(rng.rand_range(2..35), rng.rand_range(2..35));

            if terrain[&d.position().into()] != Tile::EMPTY {
                if terrain[&d.position().moved(Direction::UP).into()] != Tile::EMPTY
                    && terrain[&d.position().moved(Direction::LEFT).into()] != Tile::EMPTY
                    && terrain[&d.position().moved(Direction::DOWN).into()] != Tile::EMPTY
                    && terrain[&d.position().moved(Direction::RIGHT).into()] != Tile::EMPTY
                {
                    for x in 0..3 {
                        for y in 0..3 {
                            terrain.insert(
                                Pos::new(d.position().x + x - 1, d.position().y + y - 1),
                                Tile::EMPTY,
                            );
                        }
                    }
                } else {
                    terrain.insert(Pos::new(d.position().x, d.position().y), Tile::EMPTY);
                }
            }

            diamonds.push(d);
        }

        if terrain[&Pos::new(18, 18)] != Tile::EMPTY {
            let nearest = rooms_center
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    (
                        index,
                        (((item.0 as i32 - 18).pow(2) + (item.1 as i32 - 18).pow(2)) as f32).sqrt(),
                    )
                })
                .min_by(|(_, dist1), (_, dist2)| {
                    dist1.partial_cmp(dist2).unwrap_or(Ordering::Equal)
                })
                .unwrap()
                .0;
            if rooms_center[nearest].0 < 18 {
                for x in rooms_center[nearest].0..18 + 1 {
                    terrain.insert(Pos::new(x, rooms_center[nearest].1), Tile::EMPTY);
                }
            } else {
                for x in 18..rooms_center[nearest].0 + 1 {
                    terrain.insert(Pos::new(x, rooms_center[nearest].1), Tile::EMPTY);
                }
            }
            if rooms_center[nearest].1 < 18 {
                for y in rooms_center[nearest].1..18 + 1 {
                    terrain.insert(Pos::new(18, y), Tile::EMPTY);
                }
            } else {
                for y in 18..rooms_center[nearest].1 + 1 {
                    terrain.insert(Pos::new(18, y), Tile::EMPTY);
                }
            }
        }

        for x in 17..20 {
            for y in 17..20 {
                terrain.insert(Pos::new(x, y), Tile::EMPTY);
            }
        }

        terrain.insert(Pos::new(18, 18), Tile::EXIT);

        let mut player_pos = Pos::new(0, 0);

        while terrain[&player_pos] != Tile::EMPTY {
            player_pos.x = rng.rand_range(4..33);
            player_pos.y = rng.rand_range(4..33);
        }

        Self {
            terrain,
            player: Player::new(player_pos.x, player_pos.y),
            diamonds,
        }
    }

    pub fn empty() -> Self {
        Self {
            terrain: HashMap::new(),
            player: Player::new(0, 0),
            diamonds: Vec::new(),
        }
    }

    pub fn repr(&self) -> String {
        let mut string = String::with_capacity(1369);

        for y in 0..37 {
            'lbl: for x in 0..37 {
                if self.player.position() == (x, y) {
                    string.push('J');
                    continue;
                }
                for d in &self.diamonds {
                    if d.position() == (x, y) {
                        string.push('D');
                        continue 'lbl;
                    }
                }
                string.push(match self.terrain.get(&Pos::new(x, y)).unwrap() {
                    Tile::EMPTY => ' ',
                    Tile::WALL => 'M',
                    Tile::STONE => 'P',
                    Tile::BORDER => 'B',
                    Tile::EXIT => 'S',
                });
            }
        }

        string
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.player.move_(direction);
    }

    pub fn player_pos(&self) -> Pos {
        self.player.pos
    }

    pub fn tile_at(&self, position: Pos) -> Option<Tile> {
        match self.terrain.get(&position) {
            Some(tile) => Some(*tile),
            None => None,
        }
    }

    pub fn diamonds_pos(&self) -> Vec<Pos> {
        self.diamonds.iter().map(|d| d.position()).collect()
    }

    pub fn diamonds_count(&self) -> usize {
        self.diamonds.len()
    }

    pub fn dig(&mut self, position: Pos) {
        self.terrain.insert(position, Tile::EMPTY);
    }

    pub fn pick_up_diamond(&mut self) {
        for i in 0..self.diamonds.len() {
            if self.diamonds[i].position() == self.player_pos() {
                self.diamonds.remove(i);
                break;
            }
        }
    }
}

impl Player {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            pos: Pos::new(x, y),
        }
    }

    pub fn move_(&mut self, direction: Direction) {
        self.pos = self.pos.moved(direction);
    }
}

impl Diamond {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            pos: Pos::new(x, y),
        }
    }
}

trait Entity {
    fn position(&self) -> Pos;
}

impl Entity for Player {
    fn position(&self) -> Pos {
        self.pos
    }
}

impl Entity for Diamond {
    fn position(&self) -> Pos {
        self.pos
    }
}
